use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day7::concat;

// this is about 30x slower
fn old_concat(a: i64, b: i64) -> i64 {
    let c = a.to_string() + &b.to_string();
    c.parse().unwrap()
}

pub fn bench_concat(c: &mut Criterion) {
    c.bench_function("concat", |b| {
        b.iter(|| {
            let _ = concat(black_box(1234), black_box(56789));
        })
    });
}

pub fn bench_old_concat(c: &mut Criterion) {
    c.bench_function("old_concat", |b| {
        b.iter(|| {
            let _ = old_concat(black_box(1234), black_box(56789));
        })
    });
}

pub fn bench_solve(c: &mut Criterion) {
    let data = day7::load_input();
    c.bench_function("solve", |b| {
        b.iter(|| {
            let _ = day7::p2(&data);
        })
    });
}

// unused, just there for comparison
criterion_group! {
    name = concats;
    config = Criterion::default();
    targets = bench_concat, bench_old_concat
}

criterion_group! {
    name = solve;
    config = Criterion::default().sample_size(10).measurement_time(Duration::from_secs(60));
    targets = bench_solve
}

criterion_main!(solve);
