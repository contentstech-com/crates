use std::{hint::black_box, io::Cursor};

use criterion::{criterion_group, criterion_main, Bencher, Criterion};
use lazycsv::{Csv, CsvIterItem};
use rand::{Rng, SeedableRng as _};

const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789\",";
const ROWS: usize = 100_000;
const COLS: usize = 30;
const MIN_CHARS: usize = 3;
const MAX_CHARS: usize = 100;

fn gen_random_str<T: Rng>(rng: &mut T) -> String {
    let content: String = (0..rng.gen_range(MIN_CHARS..MAX_CHARS))
        .map(|_| CHARS[rng.gen_range(0..CHARS.len())] as char)
        .collect();

    if content.contains(',') || content.contains('"') {
        format!("\"{}\"", content.replace("\"", "\"\""))
    } else {
        content
    }
}

fn prepare() -> Vec<u8> {
    let mut buf = Vec::with_capacity(ROWS * COLS * ((MAX_CHARS - MIN_CHARS) / 2 + MIN_CHARS));

    let mut rng = rand::rngs::StdRng::from_seed(b"f3a90c67b3ca86afd62658c1b30f1f12".to_owned());
    for _ in 0..ROWS {
        for col in 0..COLS {
            buf.extend_from_slice(gen_random_str(&mut rng).as_bytes());
            if col != 29 {
                buf.push(b',');
            }
        }
        buf.push(b'\n');
    }

    buf
}

pub fn lazy_csv(b: &mut Bencher, slice: &[u8]) {
    b.iter(|| {
        for item in Csv::new(slice) {
            if let CsvIterItem::Cell(cell) = item {
                black_box(cell.try_as_str().unwrap());
            }
        }
    })
}

pub fn lazy_csv_into_rows(b: &mut Bencher, slice: &[u8]) {
    b.iter(|| {
        for row in Csv::new(slice).into_rows::<COLS>() {
            for cell in row.unwrap() {
                black_box(cell.try_as_str().unwrap());
            }
        }
    })
}

pub fn lazy_csv_raw(b: &mut Bencher, slice: &[u8]) {
    b.iter(|| {
        for cell in Csv::new(slice) {
            black_box(cell);
        }
    })
}

pub fn lazy_csv_into_rows_raw(b: &mut Bencher, slice: &[u8]) {
    b.iter(|| {
        for row in Csv::new(slice).into_rows::<COLS>() {
            for cell in row.unwrap() {
                black_box(cell);
            }
        }
    })
}

pub fn csv(b: &mut Bencher, slice: &[u8]) {
    b.iter(|| {
        let cursor = Cursor::new(slice);
        for row in csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(cursor)
            .into_records()
        {
            for cell in row.unwrap().into_iter() {
                black_box(cell);
            }
        }
    })
}

fn bench_parsers(c: &mut Criterion) {
    let mut group = c.benchmark_group("Parsers");

    group.sample_size(50);

    let buf = prepare();
    group.bench_with_input("lazy_csv", &buf.clone(), |b, buf| lazy_csv(b, buf));
    group.bench_with_input("lazy_csv (into_rows)", &buf.clone(), |b, buf| {
        lazy_csv_into_rows(b, buf)
    });
    group.bench_with_input("lazy_csv (raw)", &buf.clone(), |b, buf| {
        lazy_csv_raw(b, buf)
    });
    group.bench_with_input("lazy_csv (into_rows, raw)", &buf.clone(), |b, buf| {
        lazy_csv_into_rows_raw(b, buf)
    });
    group.bench_with_input("csv", &buf.clone(), |b, buf| csv(b, buf));
    group.finish();
}

criterion_group!(benches, bench_parsers);
criterion_main!(benches);
