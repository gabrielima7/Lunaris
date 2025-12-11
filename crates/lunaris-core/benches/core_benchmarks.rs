//! Core benchmarks for Lunaris Engine

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lunaris_core::{id::Id, time::Time};

fn id_generation_benchmark(c: &mut Criterion) {
    c.bench_function("id_generation", |b| {
        b.iter(|| {
            black_box(Id::new());
        });
    });
}

fn time_update_benchmark(c: &mut Criterion) {
    let mut time = Time::new();

    c.bench_function("time_update", |b| {
        b.iter(|| {
            time.update();
            black_box(time.delta_seconds());
        });
    });
}

fn id_batch_generation(c: &mut Criterion) {
    c.bench_function("id_batch_1000", |b| {
        b.iter(|| {
            let ids: Vec<Id> = (0..1000).map(|_| Id::new()).collect();
            black_box(ids);
        });
    });
}

criterion_group!(
    benches,
    id_generation_benchmark,
    time_update_benchmark,
    id_batch_generation
);

criterion_main!(benches);
