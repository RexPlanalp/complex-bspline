use num_complex::Complex64;

pub fn find_best_r0(knots: &[f64], r0: f64) -> f64 {
    knots
        .iter()
        .copied()
        .filter(|z| !z.is_nan())
        .min_by(|a, b| {
            let da = (a - r0).abs();
            let db = (b - r0).abs();
            da.partial_cmp(&db).unwrap()
        })
        .unwrap()
}

pub fn ecs_x(x: f64, r0: f64, eta: f64) -> Complex64 {
    if x < r0 {
        Complex64::from(x)
    } else {
        r0 + (x - r0) * Complex64::new(0.0, eta).exp()
    }
}
