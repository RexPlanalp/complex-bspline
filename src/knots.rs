use num_complex::Complex64;
use std::fs::File;
use std::io::{BufWriter, Write};

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
    pub fn new(mut config: KnotVectorConfig) -> Self {
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

        config.r0 = Self::find_best_r0(&knots, config.r0);

        let knots: Vec<Complex64> = knots.iter().map(|x| if x.re < config.r0 {*x} else {Complex64::new(config.r0, 0.0) + (x.re - config.r0) * Complex64::new(0.0, config.eta).exp()}).collect();

        Self { knots, config }
    }

    pub fn dump(&self) -> std::io::Result<()> {
    let output_file = File::create("knots.txt")?;
    let mut writer = BufWriter::new(output_file);

    for x in &self.knots {
        writeln!(writer, "{} {}", x.re, x.im)?;
    }

    Ok(())
    }

    fn find_best_r0(knots: &Vec<Complex64>, r0: f64) -> f64 {
        let mut best_match = knots[0].re;

        for x in &knots[1..] {
            if (x.re - r0).abs() < (best_match - r0).abs() {
                best_match = x.re;
            }
        }

        best_match
    }
}