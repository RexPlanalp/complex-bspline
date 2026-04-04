use crate::bsplines::basis::BSplineBasis;
use crate::knots::complex::{ComplexKnotVector, ComplexKnotConfig, EcsConfig};
use crate::knots::knot_vector::KnotVector;
use crate::bsplines::real::BSplineBasisConfig;
use crate::knots::real::{KnotConfig, RealKnotVector};
use num_complex::Complex64;
use crate::ecs::{ecs_x, find_best_r0};
use crate::bsplines::math::{b_internal, db_internal};

pub struct ComplexBSplineBasisConfig {
    pub config: BSplineBasisConfig,
    pub ecs_config: EcsConfig,
}

pub struct ComplexBSplineBasis {
    knot_vector: ComplexKnotVector,
    config: ComplexBSplineBasisConfig,
    degree: usize,
}

impl BSplineBasis<Complex64> for ComplexBSplineBasis {
    type Config = ComplexBSplineBasisConfig;
    type KV = ComplexKnotVector;

    fn new(mut config: Self::Config) -> Self {
        let degree = config.config.order - 1;

        let knot_config = KnotConfig {
            n_knots: config.config.n_basis + config.config.order,
            multiplicity: config.config.order - 1,
            start: config.config.start,
            end: config.config.end,
        };

        let real_knot_vector = RealKnotVector::build(KnotConfig {
            n_knots: knot_config.n_knots,
            multiplicity: knot_config.multiplicity,
            start: knot_config.start,
            end: knot_config.end,
        });

        config.ecs_config.r0 =
            find_best_r0(real_knot_vector.get_knots(), config.ecs_config.r0);

        let complex_knot_config = ComplexKnotConfig {
            knot_config,
            ecs_config: EcsConfig {
                r0: config.ecs_config.r0,
                eta: config.ecs_config.eta,
            },
        };

        let complex_knot_vector = ComplexKnotVector::build(complex_knot_config);

        Self {
            knot_vector: complex_knot_vector,
            config,
            degree,
        }
    }

    fn b(&self, i: usize, x: f64) -> Complex64 {
        let x_complex = ecs_x(x, self.config.ecs_config.r0, self.config.ecs_config.eta);
        b_internal(i, x_complex, &self.knot_vector, self.degree)
    }

    fn db(&self, i: usize, x: f64) -> Complex64 {
        let x_complex = ecs_x(x, self.config.ecs_config.r0, self.config.ecs_config.eta);
        db_internal(i, x_complex, &self.knot_vector, self.degree)
    }

    fn get_knot_vector(&self) -> &Self::KV {
        &self.knot_vector
    }

    fn get_n_basis(&self) -> usize {
        self.config.config.n_basis
    }
}
