use std::{fs::File, hint::black_box, io::Cursor};

use criterion::{criterion_group, criterion_main, Bencher, BenchmarkId, Criterion};
use lazycsv::{Csv, CsvIterItem};
use memchr::memchr_iter;
use memmap2::Mmap;

fn prepare(rows: usize) -> Vec<u8> {
    let f = File::open(std::env::var("INPUT").unwrap()).unwrap();
    let mmap = unsafe { Mmap::map(&f).unwrap() };
    let mut lf_iter = memchr_iter(b'\n', &mmap);
    let second_lf = lf_iter.nth(1).unwrap();
    let ending_lf = lf_iter.nth(rows).unwrap();
    let range = (second_lf + 1)..ending_lf;
    let mut vec = Vec::with_capacity(range.len());
    vec.extend_from_slice(&mmap[range]);
    vec
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
        for row in Csv::new(slice).into_rows::<28>() {
            for cell in row {
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
        for row in Csv::new(slice).into_rows::<28>() {
            for cell in row {
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
    for i in [1_000, 10_000, 50_000, 100_000] {
        group.bench_with_input(BenchmarkId::new("lazy_csv", i), &i, |b, i| {
            lazy_csv(b, &prepare(*i))
        });
        group.bench_with_input(BenchmarkId::new("lazy_csv (into_rows)", i), &i, |b, i| {
            lazy_csv_into_rows(b, &prepare(*i))
        });
        group.bench_with_input(BenchmarkId::new("lazy_csv (raw)", i), &i, |b, i| {
            lazy_csv_raw(b, &prepare(*i))
        });
        group.bench_with_input(
            BenchmarkId::new("lazy_csv (into_rows, raw)", i),
            &i,
            |b, i| lazy_csv_into_rows_raw(b, &prepare(*i)),
        );
        group.bench_with_input(BenchmarkId::new("csv", i), &i, |b, i| csv(b, &prepare(*i)));
    }
    group.finish();
}

criterion_group!(benches, bench_parsers);
criterion_main!(benches);
