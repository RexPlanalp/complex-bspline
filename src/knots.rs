use num_complex::Complex64;

#[derive(Debug)]
pub struct KnotVectorConfig{
    pub start: f64,
    pub end: f64,
    pub n: usize,
    pub r0: f64,
    pub eta: f64,
    pub multiplicity: usize,
}

#[derive(Debug)]
pub struct KnotVector{
    knots: Vec<Complex64>,
    config: KnotVectorConfig
}

impl KnotVector {
    pub fn new(config: KnotVectorConfig) -> Self {
        let mut knots: Vec<Complex64> = Vec::with_capacity(config.n);
        let n_middle = config.n - 2 * config.multiplicity;
        let step: f64 = (config.end - config.start) / ((n_middle - 1) as f64);

        for _ in 0..(config.multiplicity-1) {
            knots.push(Complex64::from(config.start));
        }

        for idx in 0..n_middle {
            knots.push(Complex64::from(config.start + (idx as f64) * step));
        }

        for _ in 0..(config.multiplicity-1) {
            knots.push(Complex64::from(config.end));
        }

        Self { knots, config }
    }
}