pub fn arange(start: f64, end: f64, step: f64) -> Vec<f64> {
    let span = end - start;

    if span == 0.0 {
        return vec![start];
    }

    let n_intervals = (span / step).round().max(1.0) as usize;
    let actual_step = span / n_intervals as f64;

    let mut out = Vec::with_capacity(n_intervals + 1);
    for i in 0..=n_intervals {
        out.push(start + i as f64 * actual_step);
    }

    return out;
}