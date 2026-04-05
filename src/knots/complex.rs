use crate::ecs::{ecs_x, find_best_r0};
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

impl KnotVector for ComplexKnotVector {
    type Scalar = Complex64;
    type Config = ComplexKnotConfig;

    fn build(mut config: Self::Config) -> Self {
        let knots = Self::build_linear_knots(
            config.knot_config.n_knots,
            config.knot_config.multiplicity,
            config.knot_config.start,
            config.knot_config.end,
        );

        config.ecs_config.r0 = find_best_r0(&knots, config.ecs_config.r0);
        let complex_knots: Vec<Complex64> = knots
            .iter()
            .map(|x| ecs_x(x.re(), config.ecs_config.r0, config.ecs_config.eta))
            .collect();

        Self {
            knots: complex_knots,
            config,
        }
    }

    fn get_knots(&self) -> &[Complex64] {
        &self.knots
    }

    fn get_start(&self) -> f64 {
        self.config.knot_config.start
    }

    fn get_end(&self) -> f64 {
        self.config.knot_config.end
    }
}
