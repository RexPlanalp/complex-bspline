

pub fn arange(start: f64, end: f64, step: f64, inclusive: bool) -> Vec<f64> {
    let span = end - start;

    if span == 0.0 {
        return vec![start];
    }

    let n_intervals = (span / step).round().max(1.0) as usize;
    let actual_step = span / n_intervals as f64;

    let mut out = Vec::with_capacity(n_intervals + 1);

    let (a, b) = if inclusive {
        (0, n_intervals + 1)
    } else {
        (1, n_intervals)
    };

    for i in a..b {
        out.push(start + i as f64 * actual_step);
    }

    return out;
}
