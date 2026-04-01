use crate::util::ecs_x;
use num_complex::Complex64;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::ops::Index;

#[derive(Debug)]
pub struct KnotVectorConfig {
    pub start: f64,
    pub end: f64,
    pub n: usize,
    pub r0: f64,
    pub eta: f64,
    pub multiplicity: usize,
}

#[derive(Debug)]
pub struct KnotVector {
    knots: Vec<Complex64>,
    pub config: KnotVectorConfig,
}

impl KnotVector {
    pub fn new(mut config: KnotVectorConfig) -> Self {
        assert!(
            config.n >= 2 * config.multiplicity,
            "numbner of knots must be at least 2 * multiplicity"
        );
        assert!(config.start <= config.end, "start must be <= end");

        let n_middle = config.n - 2 * config.multiplicity;
        let step: f64 = (config.end - config.start) / ((n_middle + 1) as f64);

        let real_knots: Vec<_> = std::iter::repeat_n(config.start, config.multiplicity)
            .map(Complex64::from)
            .chain((1..=n_middle).map(|i| Complex64::from(config.start + i as f64 * step)))
            .chain(std::iter::repeat_n(
                Complex64::from(config.end),
                config.multiplicity,
            ))
            .collect();

        config.r0 = Self::find_best_r0(&real_knots, config.r0);

        let complex_knots: Vec<Complex64> = real_knots
            .iter()
            .map(|x| ecs_x(x.re, config.r0, config.eta))
            .collect();

        Self {
            knots: complex_knots,
            config,
        }
    }

    pub fn in_interval(&self, x: f64, i: usize) -> bool {
        x >= self.knots[i].re && x < self.knots[i + 1].re
    }

    pub fn dump(&self) -> std::io::Result<()> {
        let output_file = File::create("knots.txt")?;
        let mut writer = BufWriter::new(output_file);

        for x in &self.knots {
            writeln!(writer, "{} {}", x.re, x.im)?;
        }

        Ok(())
    }

    fn find_best_r0(knots: &[Complex64], r0: f64) -> f64 {
        knots
            .iter()
            .filter(|z| !z.re.is_nan())
            .min_by(|a, b| {
                let da = (a.re - r0).abs();
                let db = (b.re - r0).abs();
                da.partial_cmp(&db).unwrap()
            })
            .map(|z| z.re)
            .unwrap()
    }
}

impl Index<usize> for KnotVector {
    type Output = Complex64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.knots[index]
    }
}
