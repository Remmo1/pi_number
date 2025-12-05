use criterion::{criterion_group, criterion_main, Criterion};

fn bench_leibniz(c: &mut Criterion) {
    c.bench_function("leibniz_pi 100M", |b| {
        b.iter(|| my_project::leibniz_pi(100_000_000))
    });
}

fn bench_leibniz_parallel(c: &mut Criterion) {
    let mut group = c.benchmark_group("leibniz_pi_parallel");

    for threads in [2, 4, 6, 8] {
        group.bench_with_input(format!("{threads} threads"), &threads, |b, &t| {
            b.iter(|| my_project::leibniz_pi_parallel_manual(100_000_000, t))
        });
    }

    group.finish();
}


criterion_group!(benches, bench_leibniz_parallel);
criterion_main!(benches);
