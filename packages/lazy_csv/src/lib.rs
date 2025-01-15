use std::{
    borrow::Cow,
    hash::{Hash, Hasher},
    mem::MaybeUninit,
};

pub struct Csv<'a, const SEP: u8 = b','> {
    buf: &'a [u8],
    state: IterState,
}

impl<'a> Csv<'a> {
    pub fn new(buf: &'a [u8]) -> Csv<'a> {
        Csv {
            buf,
            state: IterState::Cell(0),
        }
    }

    pub fn new_with_separator<const SEP: u8>(buf: &'a [u8]) -> Csv<'a, SEP> {
        Csv {
            buf,
            state: IterState::Cell(0),
        }
    }
}

impl<'a, const SEP: u8> Csv<'a, SEP> {
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

pub enum CsvIterItem<'a> {
    Cell(Cell<'a>),
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
                            match memchr::memchr3(SEP, b'\n', b'"', &self.buf[cursor..]) {
                                Some(index_relative) => {
                                    let index = index_relative + cursor;
                                    // SAFETY: since `memchr` guarantees that `index_relative` is within the bounds of `self.buf[cursor..]`, it's also guaranteed that `index_relative + cursor` is within the bounds of `self.buf`.
                                    let c = unsafe { *self.buf.get_unchecked(index) };
                                    if c == b'"' {
                                        state = State::Quoted;
                                        cursor = index + 1;
                                        padding = 1;
                                    } else {
                                        let is_crlf = c == b'\n'
                                            && index != 0
                                            // SAFETY: `index - 1` is checked to be within the bounds of `self.buf`.
                                            && unsafe { *self.buf.get_unchecked(index - 1) }
                                                == b'\r';
                                        let padding_end = padding + (is_crlf as usize);
                                        let cell = Cell {
                                            buf: &self.buf
                                                [(start + padding)..(index - padding_end)],
                                        };
                                        self.state = match c == b'\n' {
                                            true => IterState::LineEnd(index),
                                            false => IterState::Cell(index + 1),
                                        };
                                        break Some(CsvIterItem::Cell(cell));
                                    }
                                }
                                None => {
                                    self.state = IterState::Done;
                                    break None;
                                }
                            }
                        }
                        State::Quoted => match memchr::memchr(b'"', &self.buf[cursor..]) {
                            Some(index_relative) => {
                                state = State::Initial;
                                cursor = cursor + index_relative + 1;
                            }
                            None => {
                                self.state = IterState::Done;
                                break None;
                            }
                        },
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

#[derive(Debug, Clone, Eq)]
pub struct Cell<'a> {
    pub buf: &'a [u8],
}

impl<'a> Cell<'a> {
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
