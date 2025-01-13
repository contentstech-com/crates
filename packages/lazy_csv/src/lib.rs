use std::{
    borrow::Cow,
    hash::{Hash, Hasher},
    mem::MaybeUninit,
};

#[cfg(feature = "bumpalo")]
use bumpalo::{
    collections::{String as BumpString, Vec as BumpVec},
    Bump,
};

pub struct Csv<'a> {
    buf: &'a [u8],
    start: Option<usize>,
    line_end: bool,
}

impl<'a> Csv<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self {
            buf,
            start: Some(0),
            line_end: false,
        }
    }

    pub fn into_rows<const COLS: usize>(self) -> CsvRowIter<'a, COLS> {
        CsvRowIter { csv: self }
    }
}

enum State {
    Initial,
    Quoted,
}

pub enum CsvIterItem<'a> {
    Cell(Cell<'a>),
    LineEnd,
}

impl<'a> Iterator for Csv<'a> {
    type Item = CsvIterItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.line_end {
            self.line_end = false;
            return Some(CsvIterItem::LineEnd);
        }

        let start = self.start?;
        let mut cursor = start;
        let mut padding = 0;
        let mut state = State::Initial;

        loop {
            match state {
                State::Initial => match memchr::memchr3(b',', b'\n', b'"', &self.buf[cursor..]) {
                    Some(index_relative) => {
                        let index = index_relative + cursor;
                        // SAFETY: since `memchr` guarantees that `index_relative` is within the bounds of `self.buf[cursor..]`, it's also guaranteed that `index_relative + cursor` is within the bounds of `self.buf`.
                        match unsafe { *self.buf.get_unchecked(index) } {
                            c @ (b',' | b'\n') => {
                                let cell = Cell {
                                    buf: &self.buf[(start + padding)..(index - padding)],
                                    quoted: padding > 0,
                                };
                                self.start = Some(index + 1);
                                self.line_end = c == b'\n';
                                break Some(CsvIterItem::Cell(cell));
                            }
                            b'"' => {
                                state = State::Quoted;
                                cursor = index + 1;
                                padding = 1;
                            }
                            _ => unreachable!(),
                        }
                    }
                    None => break None,
                },
                State::Quoted => match memchr::memchr(b'"', &self.buf[cursor..]) {
                    Some(index_relative) => {
                        state = State::Initial;
                        cursor = cursor + index_relative + 1;
                    }
                    None => break None,
                },
            }
        }
    }
}

pub struct CsvRowIter<'a, const COLS: usize> {
    csv: Csv<'a>,
}

impl<'a, const COLS: usize> Iterator for CsvRowIter<'a, COLS> {
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

#[derive(Debug, Clone, Eq)]
pub struct Cell<'a> {
    pub buf: &'a [u8],
    pub quoted: bool,
}

impl<'a> Cell<'a> {
    pub fn try_as_str(&self) -> Result<Cow<'a, str>, std::str::Utf8Error> {
        std::str::from_utf8(self.buf).map(|s| {
            if self.quoted {
                Cow::Owned(s.replace("\"\"", "\""))
            } else {
                Cow::Borrowed(s)
            }
        })
    }

    #[cfg(feature = "bumpalo")]
    pub fn try_as_bump_string<'b: 'a>(
        &'a self,
        bump: &'b Bump,
    ) -> Result<BumpString<'b>, bumpalo::collections::string::FromUtf8Error<'b>> {
        let mut vec = BumpVec::with_capacity_in(self.buf.len(), bump);
        vec.extend_from_slice(self.buf);
        BumpString::from_utf8(vec)
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
