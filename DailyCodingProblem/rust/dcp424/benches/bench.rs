use criterion::{criterion_group, criterion_main, Criterion};
use dcp424::{appear_once, appear_once1, appear_once2};
use std::hint::black_box;

fn appear_once_benchmark(c: &mut Criterion) {
    let data = [2, 4, 6, 8, 10, 2, 6, 10];
    c.bench_function("appear_once", |b| b.iter(|| appear_once(black_box(&data))));
}

fn appear_once1_benchmark(c: &mut Criterion) {
    let data = [2, 4, 6, 8, 10, 2, 6, 10];
    c.bench_function("appear_once1", |b| {
        b.iter(|| appear_once1(black_box(&data)))
    });
}

fn appear_once2_benchmark(c: &mut Criterion) {
    let data = [2, 4, 6, 8, 10, 2, 6, 10];
    c.bench_function("appear_once2", |b| {
        b.iter(|| appear_once2(black_box(&data)))
    });
}

criterion_group!(
    benches,
    appear_once_benchmark,
    appear_once1_benchmark,
    appear_once2_benchmark
);
criterion_main!(benches);
