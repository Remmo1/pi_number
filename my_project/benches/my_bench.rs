use criterion::{criterion_group, criterion_main, Criterion};

fn bench_my_method(c: &mut Criterion) {
    c.bench_function("leibniz_pi 100M", |b| {
        b.iter(|| my_project::leibniz_pi(100_000_000))
    });
}

criterion_group!(benches, bench_my_method);
criterion_main!(benches);
