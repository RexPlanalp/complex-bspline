use crate::knots::{knot_vector::KnotVector, real::KnotConfig};
use num_complex::{Complex64, ComplexFloat};
pub struct EcsConfig {
    pub r0: f64,
    pub eta: f64,
}

pub struct ComplexKnotConfig {
    pub knot_config: KnotConfig,
    pub ecs_config: EcsConfig,
}

pub struct ComplexKnotVector {
    knots: Vec<Complex64>,
    config: ComplexKnotConfig,
}

impl ComplexKnotVector {
    fn find_best_r0(knots: &[f64], r0: f64) -> f64 {
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
}

impl KnotVector<Complex64> for ComplexKnotVector {
    type Config = ComplexKnotConfig;

    fn build(mut config: Self::Config) -> Self {
        let knots = Self::build_linear_knots(
            config.knot_config.n_knots,
            config.knot_config.multiplicity,
            config.knot_config.start,
            config.knot_config.end,
        );

        config.ecs_config.r0 = Self::find_best_r0(&knots, config.ecs_config.r0);
        let complex_knots: Vec<Complex64> = knots
            .iter()
            .map(|x| Self::ecs_x(x.re(), config.ecs_config.r0, config.ecs_config.eta))
            .collect();

        Self {
            knots: complex_knots,
            config,
        }
    }

    fn get_knots(&self) -> &[Complex64] {
        &self.knots
    }

    fn get_outfile(&self) -> &'static str {
        "complex_knots.txt"
    }

    fn get_start(&self) -> f64 {
        self.config.knot_config.start
    }

    fn get_end(&self) -> f64 {
        self.config.knot_config.end
    }
}
