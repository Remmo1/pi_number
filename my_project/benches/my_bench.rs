use criterion::{criterion_group, criterion_main, Criterion};
use std::thread;

fn bench_leibniz(c: &mut Criterion) {
    c.bench_function("leibniz_pi 100M", |b| {
        b.iter(|| my_project::leibniz_pi(100_000_000))
    });
}

fn bench_leibniz_multi_threads(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel_run");

        for threads in [1, 2, 4, 6, 50] {
            group.bench_with_input(format!("{threads} threads"), &threads, |b, &t| {
                b.iter(|| {
                    // Spawn t threads running the SAME method
                    let handles: Vec<_> = (0..t)
                        .map(|_| {
                            thread::spawn(|| {
                                my_project::leibniz_pi(100_000_000);
                            })
                        })
                        .collect();

                    // Join all threads to wait for them
                    for h in handles {
                        h.join().unwrap();
                    }
                });
            });
        }

        group.finish();
}

criterion_group!(benches, bench_leibniz_multi_threads);
criterion_main!(benches);
