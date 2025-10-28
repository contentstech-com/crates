use lazycsv::Csv;
#[cfg(feature = "alloc")]
use lazycsv::{Cell, CsvIterItem};

#[cfg(feature = "alloc")]
#[test]
fn basic() {
    let mut csv = Csv::new(
        br#"cell 1,cell 2,cell 3,cell 4
"Hello, world!","Hi ""Quote""","""HELLO""","""name"""
"#,
    );

    macro_rules! t {
        ($buf:expr, $str:expr) => {
            match csv.next() {
                Some(CsvIterItem::Cell(cell @ Cell { buf: $buf })) => {
                    assert_eq!($str, cell.try_as_str().unwrap());
                }
                other => {
                    panic!("Expected {:?}, got {:?}", $buf, other)
                }
            }
        };
        () => {
            match csv.next() {
                Some(CsvIterItem::LineEnd) => {}
                other => {
                    panic!("Expected LineEnd, got {:?}", other)
                }
            }
        };
    }

    t!(br#"cell 1"#, r#"cell 1"#);
    t!(br#"cell 2"#, r#"cell 2"#);
    t!(br#"cell 3"#, r#"cell 3"#);
    t!(br#"cell 4"#, r#"cell 4"#);
    t!();
    t!(br#""Hello, world!""#, r#"Hello, world!"#);
    t!(br#""Hi ""Quote""""#, r#"Hi "Quote""#);
    t!(br#""""HELLO""""#, r#""HELLO""#);
    t!(br#""""name""""#, r#""name""#);
    t!();
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

    assert_eq!(csv.position(), 0); // Start position

    let _ = csv.next(); // Yields Cell('aaa')
    assert_eq!(csv.position(), 4); // Position after 'aaa,' (start of 'b')

    let _ = csv.next(); // Yields Cell('bbb')
    assert_eq!(csv.position(), 7); // Position after 'bbb' (start of '\r')

    let _ = csv.next(); // Yields LineEnd
    assert_eq!(csv.position(), 8); // Position after '\n' (start of '1')

    let _ = csv.next(); // Yields Cell('100')
    assert_eq!(csv.position(), 12); // Position after '100,' (start of '2')

    let _ = csv.next(); // Yields Cell('200')
    assert_eq!(csv.position(), 15); // Position after '200' (end of buffer)

    assert!(csv.next().is_none()); // End of iteration
    assert_eq!(csv.position(), data.len()); // Position at the end
}

#[cfg(feature = "alloc")]
#[test]
fn into_rows_with_range() {
    let data = b"a,b,c\n1,2,3\n4,5,6\n";
    let csv = Csv::new(data);
    let mut iter = csv.into_rows_with_range();

    let ([a, b, c], range) = iter.next().unwrap().unwrap();
    assert_eq!(a.try_as_str().unwrap(), "a");
    assert_eq!(b.try_as_str().unwrap(), "b");
    assert_eq!(c.try_as_str().unwrap(), "c");
    assert_eq!(range, 0..6); // "a,b,c\n"

    let ([a, b, c], range) = iter.next().unwrap().unwrap();
    assert_eq!(a.try_as_str().unwrap(), "1");
    assert_eq!(b.try_as_str().unwrap(), "2");
    assert_eq!(c.try_as_str().unwrap(), "3");
    assert_eq!(range, 6..12); // "1,2,3\n"

    let ([a, b, c], range) = iter.next().unwrap().unwrap();
    assert_eq!(a.try_as_str().unwrap(), "4");
    assert_eq!(b.try_as_str().unwrap(), "5");
    assert_eq!(c.try_as_str().unwrap(), "6");
    assert_eq!(range, 12..18); // "4,5,6\n"

    assert!(iter.next().is_none());
}
