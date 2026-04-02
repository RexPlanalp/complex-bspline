use num_complex::{ComplexFloat, Complex64};
use std::io::{BufWriter, Write};
use std::fs::File;

pub trait KnotVector<T: ComplexFloat<Real = f64>> {
    type Config;

    // Public
    fn build(config: Self::Config) -> Self;

    fn get_knots(&self) -> &[T];

    fn get_outfile(&self) -> &'static str {
        "knots.txt"
    }

    fn dump(&self) -> std::io::Result<()> {
        let output_file = File::create(self.get_outfile())?;
        let mut writer = BufWriter::new(output_file);

        for x in self.get_knots() {
            writeln!(writer, "{} {}", x.re(), x.im())?;
        }

        Ok(())
    }

    fn in_interval(&self, x: f64, i: usize) -> bool {
        x >= self.get_knots()[i].re() && x < self.get_knots()[i + 1].re()
    }

    // Private
    fn validate_knot_config(n_knots: usize, multiplicity: usize, start: f64, end: f64) {
        assert!(
            n_knots >= 2 * multiplicity,
            "number of knots must be at least 2 * multiplicity"
        );
        assert!(start <= end, "start must be <= end");
    }

    fn build_linear_knots(n_knots: usize, multiplicity: usize, start: f64, end: f64) -> Vec<f64> {
        Self::validate_knot_config(n_knots, multiplicity, start, end);
        let n_middle = n_knots - 2 * multiplicity;
        let step = (end - start) / ((n_middle + 1) as f64);

        std::iter::repeat_n(start, multiplicity)
            .chain((1..=n_middle).map(|i| start + i as f64 * step))
            .chain(std::iter::repeat_n(end, multiplicity))
            .collect()
    }
}
pub struct KnotConfig {
    pub n_knots: usize,
    pub multiplicity: usize,
    pub start: f64,
    pub end: f64,

}
#[derive(Debug)]
pub struct RealKnotVector {
    knots: Vec<f64>
}

impl KnotVector<f64> for RealKnotVector {
    type Config = KnotConfig;

    fn build(config: Self::Config) -> Self {

        let knots = Self::build_linear_knots(config.n_knots, config.multiplicity, config.start, config.end);

        Self{ knots }
    }

    fn get_knots(&self) -> &[f64] {
        &self.knots
    }
}
///////////////////////////////////////////////////////////////////////////////////////////
pub struct EcsConfig {
    pub r0: f64,
    pub eta: f64
}

pub struct ComplexKnotConfig {
    pub knot_config: KnotConfig,
    pub ecs_config: EcsConfig
}

pub struct ComplexKnotVector {
    knots: Vec<Complex64>
}

impl ComplexKnotVector {
    fn find_best_r0(knots: &[f64], r0: f64) -> f64 {
        knots.iter()
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
}

impl KnotVector<Complex64> for ComplexKnotVector {
    type Config = ComplexKnotConfig;

    fn build(config: Self::Config) -> Self {
        let knots = Self::build_linear_knots(config.knot_config.n_knots, config.knot_config.multiplicity, config.knot_config.start, config.knot_config.end);

        let complex_knots: Vec<Complex64> = knots
            .iter()
            .map(|x| Self::ecs_x(x.re(), config.ecs_config.r0, config.ecs_config.eta))
            .collect();

        Self{ knots: complex_knots }
    }

    fn get_knots(&self) -> &[Complex64] {
        &self.knots
    }

    fn get_outfile(&self) -> &'static str {
        "complex_knots.txt"
    }
}
