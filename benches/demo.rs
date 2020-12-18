use criterion::{criterion_group, criterion_main, Criterion};
use criterion_demo::do_some_work;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("do_some_work", |b| b.iter(|| do_some_work()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);