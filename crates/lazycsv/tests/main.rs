use lazycsv::Csv;
#[cfg(feature = "alloc")]
use lazycsv::{Cell, CsvIterItem};

macro_rules! assert_csv {
    ($csv:expr, Cell($buf:expr)) => {
        match $csv.next() {
            Some(CsvIterItem::Cell(Cell { buf: $buf })) => (),
            other => panic!("Expected Cell, got {other:?}"),
        }
    };
    ($csv:expr, Cell($buf:expr, $str:expr)) => {
        let cell = match $csv.next() {
            Some(CsvIterItem::Cell(cell @ Cell { buf: $buf })) => cell,
            other => panic!("Expected {:?}, got {other:?}", $buf),
        };
        assert_eq!($str, cell.try_as_str().unwrap());
    };
    ($csv:expr, LineEnd) => {
        match $csv.next() {
            Some(CsvIterItem::LineEnd) => (),
            other => panic!("Expected LineEnd, got {other:?}"),
        }
    };
    ($csv:expr, EOF) => {
        match $csv.next() {
            None => (),
            other => panic!("Expected EOF, got {other:?}"),
        }
    };
    ($csv:expr, position == $expected:expr) => {
        assert_eq!($csv.position(), $expected);
    };
}

macro_rules! assert_eq_cell {
    ($cell:expr, $buf:expr) => {
        assert_eq!($cell, Cell { buf: $buf });
    };
}

#[cfg(feature = "alloc")]
#[test]
fn basic() {
    let mut csv = Csv::new(
        br#"cell 1,cell 2,cell 3,cell 4
"Hello, world!","Hi ""Quote""","""HELLO""","""name"""
"#,
    );

    assert_csv!(csv, Cell(br#"cell 1"#, r#"cell 1"#));
    assert_csv!(csv, Cell(br#"cell 2"#, r#"cell 2"#));
    assert_csv!(csv, Cell(br#"cell 3"#, r#"cell 3"#));
    assert_csv!(csv, Cell(br#"cell 4"#, r#"cell 4"#));
    assert_csv!(csv, LineEnd);
    assert_csv!(csv, Cell(br#""Hello, world!""#, r#"Hello, world!"#));
    assert_csv!(csv, Cell(br#""Hi ""Quote""""#, r#"Hi "Quote""#));
    assert_csv!(csv, Cell(br#""""HELLO""""#, r#""HELLO""#));
    assert_csv!(csv, Cell(br#""""name""""#, r#""name""#));
    assert_csv!(csv, LineEnd);
    assert_csv!(csv, EOF);
}

#[cfg(feature = "alloc")]
#[test]
fn dequote() {
    let cell = Cell {
        buf: br#""Hi ""Quote"" yo""#,
    };
    assert_eq!(cell.try_as_str().unwrap(), r#"Hi "Quote" yo"#);
}

#[test]
fn position() {
    let data = b"aaa,bbb\n100,200";
    let mut csv = Csv::new(data);

    assert_csv!(csv, position == 0); // Start position
    assert_csv!(csv, Cell(b"aaa")); // Yields Cell('aaa')
    assert_csv!(csv, position == 4); // Position after 'aaa,' (start of 'b')
    assert_csv!(csv, Cell(b"bbb")); // Yields Cell('bbb')
    assert_csv!(csv, position == 7); // Position after 'bbb' (start of '\r')
    assert_csv!(csv, LineEnd); // Yields LineEnd
    assert_csv!(csv, position == 8); // Position after '\n' (start of '1')
    assert_csv!(csv, Cell(b"100")); // Yields Cell('100')
    assert_csv!(csv, position == 12); // Position after '100,' (start of '2')
    assert_csv!(csv, Cell(b"200")); // Yields Cell('200')
    assert_csv!(csv, position == 15); // Position after '200' (end of buffer)
    assert_csv!(csv, EOF); // End of iteration
    assert_csv!(csv, position == data.len()); // Position at the end
}

#[cfg(feature = "alloc")]
#[test]
fn into_rows() {
    let mut iter = Csv::new(b"a,b,c\n1,2,3\n4,5,6\n").into_rows();

    let [a, b, c] = iter.next().unwrap().unwrap();
    assert_eq_cell!(a, b"a");
    assert_eq_cell!(b, b"b");
    assert_eq_cell!(c, b"c");

    let [a, b, c] = iter.next().unwrap().unwrap();
    assert_eq_cell!(a, b"1");
    assert_eq_cell!(b, b"2");
    assert_eq_cell!(c, b"3");

    let [a, b, c] = iter.next().unwrap().unwrap();
    assert_eq_cell!(a, b"4");
    assert_eq_cell!(b, b"5");
    assert_eq_cell!(c, b"6");

    assert!(iter.next().is_none());
}

#[cfg(feature = "alloc")]
#[test]
fn into_rows_with_range() {
    let mut iter = Csv::new(b"a,b,c\n1,2,3\n4,5,6\n").into_rows_with_range();

    let ([a, b, c], range) = iter.next().unwrap().unwrap();
    assert_eq_cell!(a, b"a");
    assert_eq_cell!(b, b"b");
    assert_eq_cell!(c, b"c");
    assert_eq!(range, 0..6); // "a,b,c\n"

    let ([a, b, c], range) = iter.next().unwrap().unwrap();
    assert_eq_cell!(a, b"1");
    assert_eq_cell!(b, b"2");
    assert_eq_cell!(c, b"3");
    assert_eq!(range, 6..12); // "1,2,3\n"

    let ([a, b, c], range) = iter.next().unwrap().unwrap();
    assert_eq_cell!(a, b"4");
    assert_eq_cell!(b, b"5");
    assert_eq_cell!(c, b"6");
    assert_eq!(range, 12..18); // "4,5,6\n"

    assert!(iter.next().is_none());
}
