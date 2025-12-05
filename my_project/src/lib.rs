use std::thread;

#[inline(never)]
pub fn leibniz_pi(n_iter: usize) -> f64 {
    let mut sum = 0.0f64;
    for k in 0..n_iter {
        let kf = k as f64;
        let term = if k % 2 == 0 { 1.0 } else { -1.0 } / (2.0 * kf + 1.0);
        sum += term;
    }
    4.0 * sum
}

#[inline(never)]
pub fn leibniz_pi_parallel_manual(n: u64, threads: usize) -> f64 {
    let chunk = n / threads as u64;

    let handles: Vec<_> = (0..threads)
        .map(|t| {
            let start = t as u64 * chunk;
            let end = if t == threads - 1 { n } else { start + chunk };

            thread::spawn(move || {
                let mut sum = 0.0;
                for k in start..end {
                    let sign = if k % 2 == 0 { 1.0 } else { -1.0 };
                    sum += sign / (2.0 * k as f64 + 1.0);
                }
                sum
            })
        })
        .collect();

    let total: f64 = handles.into_iter().map(|h| h.join().unwrap()).sum();
    4.0 * total
}
