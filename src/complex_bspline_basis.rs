use crate::bspline_basis::BSplineBasis;
use crate::knot_vector::KnotVector;
use crate::complex_knot_vector::{ComplexKnotConfig, ComplexKnotVector, EcsConfig};
use crate::real_bspline_basis::BSplineBasisConfig;
use crate::real_knot_vector::{RealKnotVector, KnotConfig};
use num_complex::Complex64;

pub struct ComplexBSplineBasisConfig {
    pub config: BSplineBasisConfig,
    pub ecs_config: EcsConfig
}

pub struct ComplexBSplineBasis {
    knot_vector: ComplexKnotVector,
    config: ComplexBSplineBasisConfig,
    degree: usize
}

impl ComplexBSplineBasis {
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

impl BSplineBasis<Complex64> for ComplexBSplineBasis {
    type Config = ComplexBSplineBasisConfig;
    type KV = ComplexKnotVector;

    fn new(mut config: Self::Config) -> Self {
        let degree = config.config.order - 1;

        let knot_config = KnotConfig {
            n_knots: config.config.n_basis + config.config.order,
            multiplicity: config.config.order - 1,
            start: config.config.start,
            end: config.config.end
        };

        let real_knot_vector = RealKnotVector::build(KnotConfig {
            n_knots: knot_config.n_knots,
            multiplicity: knot_config.multiplicity,
            start: knot_config.start,
            end: knot_config.end
        });

        config.ecs_config.r0 = Self::find_best_r0(real_knot_vector.get_knots(), config.ecs_config.r0);

        let complex_knot_config = ComplexKnotConfig {
            knot_config, 
            ecs_config: EcsConfig {
                r0: config.ecs_config.r0,
                eta: config.ecs_config.eta
            }
        };

        let complex_knot_vector = ComplexKnotVector::build(complex_knot_config);

        Self {knot_vector: complex_knot_vector, config, degree}
    }

    fn b(&self, i: usize, x: f64) -> Complex64 {
        let x_complex = Self::ecs_x(x, self.config.ecs_config.r0, self.config.ecs_config.eta);
        self.b_internal(i, x_complex, self.degree)
    }

    fn db(&self, i: usize, x: f64) -> Complex64 {
        let x_complex = Self::ecs_x(x, self.config.ecs_config.r0, self.config.ecs_config.eta);
        self.db_internal(i, x_complex, self.degree)
    }

    fn get_knot_vector(&self) -> &Self::KV {
        &self.knot_vector
    }

    fn get_n_basis(&self) -> usize {
        self.config.config.n_basis
    }

}