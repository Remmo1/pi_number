use criterion::{criterion_group, criterion_main, Criterion};

fn leibniz_pi(n_iter: usize) -> f64 {
    let mut sum = 0.0f64;
    for k in 0..n_iter {
        let kf = k as f64;
        let term = if k % 2 == 0 { 1.0 } else { -1.0 } / (2.0 * kf + 1.0);
        sum += term;
    }
    4.0 * sum
}

fn bench_my_method(c: &mut Criterion) {
    c.bench_function("leibniz_pi 100M", |b| {
        b.iter(|| leibniz_pi(100_000_000))
    });
}

criterion_group!(benches, bench_my_method);
criterion_main!(benches);
