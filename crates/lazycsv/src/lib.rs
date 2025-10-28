//! The `lazycsv` crate provides a performant CSV parser.
//!
//! [Benchmarks](https://lazycsv.contentstech.com)
//!
//! # Primary Focuses
//!
//! lazycsv is a parser that performs optimistic optimization. It’s primarily
//! optimized for parsing CSV input that is either unquoted or only minimally
//! quoted—especially when dequoting is unnecessary. In such cases, it can
//! outperform [BurntSushi/rust-csv] by around 20% in terms of performance.
//!
//! However, if the input is expected to require dequotation, it’s generally better
//! to use [BurntSushi/rust-csv], which performs eager dequoting during the parsing
//! phase. Since lazycsv is a lazy parser, it defers dequoting entirely. If
//! dequotation is performed later, this effectively results in scanning the input
//! twice, which leads to a performance penalty.
//!
//! [BurntSushi/rust-csv]: https://github.com/BurntSushi/rust-csv
//!
//! - **Vectorized**: The parser utilizes SIMD operations, therefore is very performant.
//! - **Minimal hidden costs**: Every API doesn't bring any invisible overheads, and each operation only does what it needs to do.
//! - **Zero copy, zero allocation by default**: The parser doesn't allocate any memory during parsing and only performs allocation when dequoting each cell.
//! - **Lazy Decoding**: Input is not copied or unquoted until requested. This is useful when you only need to access a few cells in a large CSV file.
//! - **`#![no_std]` eligible**: The crate is `#![no_std]` compatible, and it can be used in systems without an allocator.
//!
//! # Supported Features
//!
//! `lazycsv` primarily supports a subset of [RFC 4180](https://datatracker.ietf.org/doc/html/rfc4180) with minor extensions.
//!
//! ## According to RFC 4180:
//!
//! - No escape mechanisms other than quoting are supported.
//! - Padding cells with whitespace is not allowed.
//! - Using double quotes without quoting is not allowed.
//! - Quotes must always appear at the very beginning of a cell.
//!
//! ## Additional Restrictions:
//!
//! - Only ASCII and UTF-8 encodings are supported.
//!
//! ## Additional Supports:
//!
//! - Using LF (`\n`) instead of CRLF (`\r\n`) as the newline is permitted.
//! - Customizing the separator character is possible.
//!
//! # Examples
//!
//! ```
//! # #[cfg(feature = "alloc")]
//! # {
//! use lazycsv::{Csv, CsvIterItem};
//!
//! // Iterating over rows
//! let csv = Csv::new(b"a,b,c\n1,2,3\n");
//! for row in csv.into_rows() {
//!     let [first, second, third] = row?;
//!     println!(
//!         "{}, {}, {}",
//!         first.try_as_str()?,
//!         second.try_as_str()?,
//!         third.try_as_str()?,
//!     );
//! }
//!
//! // Or if you want to avoid buffering:
//! let csv2 = Csv::new(b"a,b,c\n1,2,3\n");
//! for item in csv2 {
//!     if let CsvIterItem::Cell(cell) = item {
//!         println!("{}", cell.try_as_str()?);
//!     }
//! }
//! # }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # Crate features
//!
//! * **std** - When enabled (the default), this will permit features specific to the standard
//!   library. Currently, the only thing used from the standard library is runtime SIMD CPU feature
//!   detection. This means that this feature must be enabled to get AVX2 accelerated routines on
//!   `x86_64` targets without enabling the `avx2` feature at compile time, for example. When `std`
//!   is not enabled, this crate will still attempt to use SSE2 accelerated routines on `x86_64`.
//!   It will also use AVX2 accelerated routines when the `avx2` feature is enabled at compile
//!   time. In general, enable this feature if you can.
//! * **alloc** - When enabled (the default), API in this crate requiring some kind of allocation
//!   will become available. (i.e. [`Cell::try_as_str`](crate::Cell::try_as_str)) Otherwise, this
//!   crate is designed from the ground up to be usable in core-only contexts, so the `alloc`
//!   feature doesn't add much currently. Notably, disabling `std` but enabling `alloc` will
//!   **not** result in the use of AVX2 on `x86_64` targets unless the `avx2` feature is enabled at
//!   compile time. (With `std` enabled, AVX2 can be used even without the `avx2` feature enabled
//!   at compile time by way of runtime CPU feature detection.)

#![no_std]
#![deny(missing_docs)]

use core::{hash::Hash, mem::MaybeUninit, ops::Range};

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::borrow::Cow;

use memchr::{memchr, memchr3};
use thiserror::Error;

/// A stateful CSV parser.
///
/// See the [crate-level documentation](crate) for more details.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Csv<'a> {
    buf: &'a [u8],
    separator: u8,
    state: IterState,
}

impl<'a> Csv<'a> {
    /// Creates a new CSV parser for the given buffer.
    ///
    /// To customize the separator character, use [`Csv::with_separator()`].
    ///
    /// # Example
    ///
    /// ```
    /// use lazycsv::Csv;
    ///
    /// let csv = Csv::new(b"a,b,c\n1,2,3\n");
    /// ```
    pub fn new(buf: &'a [u8]) -> Csv<'a> {
        Csv {
            buf,
            separator: b',',
            state: IterState::Cell(0),
        }
    }

    /// Creates a new CSV parser for the given buffer, with the given separator character.
    ///
    /// # Example
    ///
    /// ```
    /// use lazycsv::Csv;
    ///
    /// // Parsing TSV instead of CSV
    /// let tsv = Csv::with_separator(b"a\tb\tc\n1\t2\t3", b'\t');
    /// ```
    pub fn with_separator(buf: &'a [u8], separator: u8) -> Csv<'a> {
        Csv {
            buf,
            separator,
            state: IterState::Cell(0),
        }
    }

    /// Create a wrapper iterator that buffers the cells per row.
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(feature = "alloc")]
    /// # {
    /// use lazycsv::Csv;
    ///
    /// for row in Csv::new(b"a,b,c\n1,2,3\n").into_rows() {
    ///     let [first, second, third] = row?;
    ///     println!("{}, {}, {}", first.try_as_str()?, second.try_as_str()?, third.try_as_str()?);
    /// }
    /// # }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn into_rows<const COLS: usize>(self) -> CsvRowIter<'a, COLS> {
        CsvRowIter { csv: self }
    }

    /// Create a wrapper iterator that buffers the cells per row, along with byte position range.
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(feature = "alloc")]
    /// # {
    /// use lazycsv::Csv;
    ///
    /// for row in Csv::new(b"a,b,c\n1,2,3\n").into_rows_with_range() {
    ///     let ([first, second, third], range) = row?;
    ///     println!(
    ///         "{}, {}, {} (bytes {}..{})",
    ///         first.try_as_str()?,
    ///         second.try_as_str()?,
    ///         third.try_as_str()?,
    ///         range.start,
    ///         range.end
    ///     );
    /// }
    /// # }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn into_rows_with_range<const COLS: usize>(self) -> CsvRowWithRangeIter<'a, COLS> {
        CsvRowWithRangeIter {
            row_iter: self.into_rows(),
        }
    }

    /// Skips the first `n` rows.
    ///
    /// Using this function is more efficient than calling [`Iterator::skip()`] on the row iterator made with [`Csv::into_rows()`],
    /// as it only looks for newline characters instead of trying to recognize cells.
    ///
    /// # Example
    ///
    /// ```
    /// # let _: Option<()> = (|| {
    /// use lazycsv::{Csv, CsvIterItem};
    ///
    /// let mut csv = Csv::new(b"a,b,c\n1,2,3\n4,5,6\n");
    /// let CsvIterItem::Cell(cell) = csv.skip_rows(2).next()? else {
    ///     panic!("Expected a cell");
    /// };
    /// assert_eq!(cell.buf, b"4");
    /// # None
    /// # })();
    /// ```
    pub fn skip_rows(mut self, n: usize) -> Self {
        let mut start = match self.state {
            IterState::Cell(start) => start,
            IterState::LineEnd(lf, is_crlf) => lf + 1 + (is_crlf as usize),
            IterState::Done => return self,
        };

        for _ in 0..n {
            if let Some(index_relative) = memchr::memchr(b'\n', &self.buf[start..]) {
                start += index_relative + 1;
            } else {
                self.state = IterState::Done;
                break;
            };
        }
        self.state = IterState::Cell(start);
        self
    }

    /// Returns the current byte position of the parser within the input buffer.
    ///
    /// This indicates the starting position of the *next* item (cell or line break)
    /// to be parsed. If the iteration is finished, it returns the length of the buffer.
    ///
    /// # Example
    ///
    /// ```
    /// use lazycsv::{Csv, CsvIterItem};
    ///
    /// let data = b"aaa,bbb\r\n100,200";
    /// let mut csv = Csv::new(data);
    ///
    /// assert_eq!(csv.position(), 0); // Start position
    ///
    /// let _ = csv.next(); // Yields Cell('aaa')
    /// assert_eq!(csv.position(), 4); // Position after 'aaa,' (start of 'b')
    ///
    /// let _ = csv.next(); // Yields Cell('bbb')
    /// assert_eq!(csv.position(), 7); // Position after 'bbb' (start of '\r')
    ///
    /// let _ = csv.next(); // Yields LineEnd
    /// assert_eq!(csv.position(), 9); // Position after '\r\n' (start of '1')
    ///
    /// let _ = csv.next(); // Yields Cell('100')
    /// assert_eq!(csv.position(), 13); // Position after '100,' (start of '2')
    ///
    /// let _ = csv.next(); // Yields Cell('200')
    /// assert_eq!(csv.position(), 16); // Position after '200' (end of buffer)
    ///
    /// assert!(csv.next().is_none()); // End of iteration
    /// assert_eq!(csv.position(), data.len()); // Position at the end
    /// ```
    pub fn position(&self) -> usize {
        match self.state {
            IterState::Cell(pos) => pos,
            IterState::LineEnd(pos, _) => pos, // The next item starts after the newline character
            IterState::Done => self.buf.len(),
        }
    }
}

/// Expected next item in the CSV parser.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum IterState {
    Cell(usize),
    LineEnd(usize, bool),
    Done,
}

/// An item yielded by [`Csv`], indicates either a cell or a line break.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum CsvIterItem<'a> {
    /// The row continues with a cell.
    Cell(Cell<'a>),
    /// The row ends with a line break.
    LineEnd,
}

impl<'a> Iterator for Csv<'a> {
    type Item = CsvIterItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let start = match self.state {
            IterState::LineEnd(pos, is_crlf) => {
                self.state = IterState::Cell(pos + 1 + (is_crlf as usize));
                return Some(CsvIterItem::LineEnd);
            }
            IterState::Done => return None,
            IterState::Cell(start) => start,
        };

        let mut cursor = start;
        let mut in_quoted_state = false;

        loop {
            if in_quoted_state {
                let Some(index_relative) = memchr(b'"', &self.buf[cursor..]) else {
                    self.state = IterState::Done;
                    return None;
                };
                in_quoted_state = false;
                cursor += index_relative + 1;
                continue;
            }

            let Some(index_relative) = memchr3(self.separator, b'\n', b'"', &self.buf[cursor..])
            else {
                self.state = IterState::Done;
                return if start < self.buf.len() {
                    // Return the last cell if there's remaining data.
                    Some(CsvIterItem::Cell(Cell {
                        buf: &self.buf[start..],
                    }))
                } else if self.buf.ends_with(&[self.separator]) {
                    // Handle trailing empty cell when no trailing newline is present.
                    Some(CsvIterItem::Cell(Cell { buf: &[] }))
                } else {
                    // Gracefully reached EOF with no more data
                    None
                };
            };
            let index = index_relative + cursor;

            // SAFETY: since `memchr` guarantees that `index_relative` is within the bounds of
            // `self.buf[cursor..]`, it's also guaranteed that `index_relative + cursor` is within
            // the bounds of `self.buf`.
            let c = unsafe { *self.buf.get_unchecked(index) };

            if c == b'"' {
                in_quoted_state = true;
                cursor = index + 1;
                continue;
            }

            // SAFETY: `index - 1` is checked to be within the bounds of `self.buf`.
            let is_crlf =
                c == b'\n' && index != 0 && unsafe { *self.buf.get_unchecked(index - 1) } == b'\r';
            let end = index - (is_crlf as usize);
            let cell = Cell {
                buf: &self.buf[start..end],
            };
            self.state = if c == b'\n' {
                IterState::LineEnd(end, is_crlf)
            } else {
                IterState::Cell(index + 1)
            };
            return Some(CsvIterItem::Cell(cell));
        }
    }
}

/// An iterator that buffers and yields rows of cells.
///
/// Can be created by calling [`Csv::into_rows()`].
///
/// ### `const` Parameters
///
/// - `COLS`: The number of columns in the CSV.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CsvRowIter<'a, const COLS: usize> {
    csv: Csv<'a>,
}

impl<const COLS: usize> CsvRowIter<'_, COLS> {
    /// Skips the first `n` rows.
    ///
    /// Using this function is more efficient than calling [`Iterator::skip()`],
    /// as it only looks for newline characters instead of trying to recognize cells.
    ///
    /// # Example
    ///
    /// ```
    /// # let _: Option<()> = (|| {
    /// use lazycsv::Csv;
    ///
    /// let mut rows = Csv::new(b"a,b,c\n1,2,3\n4,5,6\n").into_rows();
    /// let [four, five, six] = rows.skip(2).next()?.ok()? else {
    ///     panic!("Expected a row");
    /// };
    /// assert_eq!([four.buf, five.buf, six.buf], [b"4", b"5", b"6"]);
    /// # None
    /// # })();
    /// ```
    pub fn skip(self, n: usize) -> Self {
        Self {
            csv: self.csv.skip_rows(n),
        }
    }
}

impl<'a, const COLS: usize> Iterator for CsvRowIter<'a, COLS> {
    type Item = Result<[Cell<'a>; COLS], RowIterError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut arr = [const { MaybeUninit::uninit() }; COLS];
        for i in 0..COLS {
            match self.csv.next() {
                // If we reach EOF before reading any cells, there are no more rows available.
                None if i == 0 => return None,
                Some(CsvIterItem::Cell(cell)) => {
                    // SAFETY: we have to initialize the cell beforehand
                    unsafe { arr.get_unchecked_mut(i).write(cell) };
                }
                None | Some(CsvIterItem::LineEnd) => {
                    return Some(Err(RowIterError::ColumnCountSmallerThanExpected {
                        expected: COLS,
                        actual: i,
                    }));
                }
            }
        }

        // After reading COLS cells, the next item must be a line ending or EOF.
        // EOF in this context is treated as a valid input to gracefully handle
        // files without a trailing newline.
        if let None | Some(CsvIterItem::LineEnd) = self.csv.next() {
            Some(Ok(arr.map(|mem| unsafe { mem.assume_init() })))
        } else {
            Some(Err(RowIterError::ColumnCountLargerThanExpected {
                expected: COLS,
            }))
        }
    }
}

/// An iterator that buffers and yields rows of cells along with byte position range.
///
/// Can be created by calling [`Csv::into_rows_with_range()`].
///
/// ### `const` Parameters
///
/// - `COLS`: The number of columns in the CSV.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CsvRowWithRangeIter<'a, const COLS: usize> {
    row_iter: CsvRowIter<'a, COLS>,
}

impl<const COLS: usize> CsvRowWithRangeIter<'_, COLS> {
    /// Skips the first `n` rows.
    ///
    /// Using this function is more efficient than calling [`Iterator::skip()`],
    /// as it only looks for newline characters instead of trying to recognize cells.
    ///
    /// # Example
    ///
    /// ```
    /// # let _: Option<()> = (|| {
    /// use lazycsv::Csv;
    ///
    /// let mut rows = Csv::new(b"a,b,c\n1,2,3\n4,5,6\n").into_rows_with_range();
    /// let ([four, five, six], range) = rows.skip(2).next()?.ok()? else {
    ///     panic!("Expected a row");
    /// };
    /// assert_eq!([four.buf, five.buf, six.buf], [b"4", b"5", b"6"]);
    /// assert_eq!(range, 12..18);
    /// # None
    /// # })();
    /// ```
    pub fn skip(self, n: usize) -> Self {
        Self {
            row_iter: self.row_iter.skip(n),
        }
    }
}

impl<'a, const COLS: usize> Iterator for CsvRowWithRangeIter<'a, COLS> {
    type Item = Result<([Cell<'a>; COLS], Range<usize>), RowIterError>;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.row_iter.csv.position();
        let next = self.row_iter.next();
        let end = self.row_iter.csv.position();
        next.map(|res| res.map(|arr| (arr, start..end)))
    }
}

/// Errors returned by [`CsvRowIter`].
#[derive(Error, Clone, Eq, PartialEq, Hash, Debug)]
pub enum RowIterError {
    /// Found smaller number of columns than expected.
    #[error("expected {expected} columns, but new row started after parsing {actual} columns")]
    ColumnCountSmallerThanExpected {
        /// The expected number of columns.
        expected: usize,
        /// The actual number of columns.
        actual: usize,
    },

    /// Found larger number of columns than expected.
    #[error("expected {expected} columns, but no newline found after parsing {expected} columns")]
    ColumnCountLargerThanExpected {
        /// The expected number of columns.
        expected: usize,
    },
}

/// A cell in a CSV row.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Cell<'a> {
    /// The underlying buffer, containing potentially quoted cell content as bytes.
    pub buf: &'a [u8],
}

#[cfg(feature = "alloc")]
impl<'a> Cell<'a> {
    /// Converts the cell to a string.
    ///
    /// Calling this function performs a UTF-8 validation and dequotes the cell if necessary.
    ///
    /// # Performance
    ///
    /// As noted in the [crate-level documentation][crate], the performance benefits of lazycsv are
    /// most effective when dequoting is not required. This is because lazycsv does not perform
    /// dequoting during the parsing phase, and if dequoting is later requested, it incurs a
    /// performance penalty from scanning the input twice.
    ///
    /// Therefore, when using lazycsv, it is recommended—whenever possible—to avoid dequoting and
    /// instead access the underlying buffer directly, or limit dequoting to only a small subset of
    /// cells.
    ///
    /// If that’s not feasible and extensive dequoting is necessary, consider using an eager parser
    /// like [BurntSushi/rust-csv], which performs dequoting during parsing and avoids this
    /// overhead.
    ///
    /// [BurntSushi/rust-csv]: https://github.com/BurntSushi/rust-csv
    pub fn try_as_str(&self) -> Result<Cow<'a, str>, core::str::Utf8Error> {
        core::str::from_utf8(self.buf).map(|s| {
            // SAFETY: since `s.as_bytes()` is guaranteed to be valid UTF-8, it's also guaranteed that the first character is '"' if the first byte is b'"' due to UTF-8 representing ASCII characters as-is.
            if !s.is_empty() && unsafe { *s.as_bytes().get_unchecked(0) } == b'"' {
                Cow::Owned(s[1..(s.len() - 1)].replace("\"\"", "\""))
            } else {
                Cow::Borrowed(s)
            }
        })
    }
}
