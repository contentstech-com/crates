#[cfg(feature = "alloc")]
use lazycsv::{Cell, Csv, CsvIterItem};

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
