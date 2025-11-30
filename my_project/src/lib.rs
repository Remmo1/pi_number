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
