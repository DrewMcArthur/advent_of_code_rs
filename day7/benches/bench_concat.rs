use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day7::{concat, Equation};

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
    let data = [
        Equation::from("123: 1 2 3"),
        Equation::from("456: 4 5 6"),
        Equation::from("789: 7 8 9"),
        Equation::from("101112: 10 11 12"),
        Equation::from("131415: 13 14 15"),
        Equation::from("161718: 16 17 18"),
        Equation::from("192021: 19 20 21"),
        Equation::from("1: 10 112 15 2 324 12 23 4 234 23 423"),
    ];
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
    config = Criterion::default();
    targets = bench_solve
}

criterion_main!(solve);
