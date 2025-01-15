use std::{
    borrow::Cow,
    hash::{Hash, Hasher},
    mem::MaybeUninit,
};

/// A stateful CSV parser.
///
/// `lazycsv` primarily supports a subset of [RFC 4180](https://datatracker.ietf.org/doc/html/rfc4180) with minor extensions.
///
/// ### According to RFC 4180:
///
/// - No escape mechanisms other than quoting are supported.
/// - Padding cells with whitespace is not allowed.
/// - Using double quotes without quoting is not allowed.
/// - Quotes must always appear at the very beginning of a cell.
///
/// ### Additional Restrictions:
///
/// - Only ASCII and UTF-8 encodings are supported.
///
/// ### Additional Supports:
///
/// - Using LF (`\n`) instead of CRLF (`\r\n`) as the newline is permitted.
/// - Customizing the separator character is possible by using [`Csv::new_with_separator()`].
pub struct Csv<'a, const SEP: u8 = b','> {
    buf: &'a [u8],
    state: IterState,
}

impl<'a> Csv<'a> {
    /// Creates a new CSV parser for the given buffer.
    ///
    /// To customize the separator character, use [`Csv::new_with_separator()`].
    pub fn new(buf: &'a [u8]) -> Csv<'a> {
        Csv {
            buf,
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
    /// let tsv = Csv::new_with_separator::<b'\t'>(b"a\tb\tc\n1\t2\t3");
    /// ```
    pub fn new_with_separator<const SEP: u8>(buf: &'a [u8]) -> Csv<'a, SEP> {
        Csv {
            buf,
            state: IterState::Cell(0),
        }
    }
}

impl<'a, const SEP: u8> Csv<'a, SEP> {
    /// Create a wrapper iterator that buffers the cells per row.
    ///
    /// # Example
    ///
    /// ```
    /// use lazycsv::Csv;
    ///
    /// for [first, second, third] in Csv::new(b"a,b,c\n1,2,3").into_rows() {
    ///     println!("{}, {}, {}", first.try_as_str().unwrap(), second.try_as_str().unwrap(), third.try_as_str().unwrap());
    /// }
    /// ```
    pub fn into_rows<const COLS: usize>(self) -> CsvRowIter<'a, COLS, SEP> {
        CsvRowIter { csv: self }
    }
}

enum IterState {
    Cell(usize),
    LineEnd(usize),
    Done,
}

enum State {
    Initial,
    Quoted,
}

/// An item yielded by [`Csv`], indicates either a cell or a line break.
pub enum CsvIterItem<'a> {
    /// The row continues with a cell.
    Cell(Cell<'a>),
    /// The row ends with a line break.
    LineEnd,
}

impl<'a, const SEP: u8> Iterator for Csv<'a, SEP> {
    type Item = CsvIterItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            IterState::Cell(start) => {
                let mut cursor = start;
                let mut padding = 0;
                let mut state = State::Initial;

                loop {
                    match state {
                        State::Initial => {
                            let Some(index_relative) =
                                memchr::memchr3(SEP, b'\n', b'"', &self.buf[cursor..])
                            else {
                                self.state = IterState::Done;
                                break None;
                            };
                            let index = index_relative + cursor;
                            // SAFETY: since `memchr` guarantees that `index_relative` is within the bounds of `self.buf[cursor..]`, it's also guaranteed that `index_relative + cursor` is within the bounds of `self.buf`.
                            let c = unsafe { *self.buf.get_unchecked(index) };
                            if c == b'"' {
                                state = State::Quoted;
                                cursor = index + 1;
                                padding = 1;
                            } else {
                                // SAFETY: `index - 1` is checked to be within the bounds of `self.buf`.
                                let is_crlf = c == b'\n'
                                    && index != 0
                                    && unsafe { *self.buf.get_unchecked(index - 1) } == b'\r';
                                let padding_end = padding + (is_crlf as usize);
                                let cell = Cell {
                                    buf: &self.buf[(start + padding)..(index - padding_end)],
                                };
                                self.state = match c == b'\n' {
                                    true => IterState::LineEnd(index),
                                    false => IterState::Cell(index + 1),
                                };
                                break Some(CsvIterItem::Cell(cell));
                            }
                        }
                        State::Quoted => {
                            let Some(index_relative) = memchr::memchr(b'"', &self.buf[cursor..])
                            else {
                                self.state = IterState::Done;
                                break None;
                            };
                            state = State::Initial;
                            cursor = cursor + index_relative + 1;
                        }
                    }
                }
            }
            IterState::LineEnd(pos) => {
                self.state = IterState::Cell(pos + 1);
                Some(CsvIterItem::LineEnd)
            }
            IterState::Done => None,
        }
    }
}

/// An iterator that buffers and yields rows of cells.
///
/// Can be created by calling [`Csv::into_rows()`].
pub struct CsvRowIter<'a, const COLS: usize, const SEP: u8> {
    csv: Csv<'a, SEP>,
}

impl<'a, const COLS: usize, const SEP: u8> Iterator for CsvRowIter<'a, COLS, SEP> {
    type Item = [Cell<'a>; COLS];

    fn next(&mut self) -> Option<Self::Item> {
        let mut arr = [const { MaybeUninit::uninit() }; COLS];
        for i in 0..COLS {
            match self.csv.next() {
                Some(CsvIterItem::Cell(cell)) => {
                    // SAFETY: we have to initialize the cell beforehand
                    unsafe { arr.get_unchecked_mut(i).write(cell) };
                }
                Some(CsvIterItem::LineEnd) => panic!("Column count doesn't match: expected {COLS} columns, but new row started at index {i}"),
                None => return None,
            }
        }

        if !matches!(self.csv.next(), Some(CsvIterItem::LineEnd)) {
            panic!("Column count doesn't match: expected {COLS} columns, but cell still exists");
        }

        Some(arr.map(|mem| unsafe { mem.assume_init() }))
    }
}

/// A cell in a CSV row.
#[derive(Debug, Clone, Eq)]
pub struct Cell<'a> {
    pub buf: &'a [u8],
}

impl<'a> Cell<'a> {
    /// Converts the cell to a string.
    ///
    /// Calling this function performs a UTF-8 validation and dequotes the cell if necessary.
    pub fn try_as_str(&self) -> Result<Cow<'a, str>, std::str::Utf8Error> {
        std::str::from_utf8(self.buf).map(|s| {
            // SAFETY: since `s.as_bytes()` is guaranteed to be valid UTF-8, it's also guaranteed that the first character is '"' if the first byte is b'"' due to UTF-8 representing ASCII characters as-is.
            if !s.is_empty() && unsafe { *s.as_bytes().get_unchecked(0) } == b'"' {
                Cow::Owned(s.replace("\"\"", "\""))
            } else {
                Cow::Borrowed(s)
            }
        })
    }
}

impl Hash for Cell<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.buf.hash(state);
    }
}

impl PartialEq for Cell<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.buf == other.buf
    }
}

impl PartialOrd for Cell<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.buf.cmp(other.buf))
    }
}

impl Ord for Cell<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.buf.cmp(other.buf)
    }
}
