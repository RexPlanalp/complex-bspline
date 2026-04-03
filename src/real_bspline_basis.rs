use crate::bspline_basis::BSplineBasis;
use crate::knot_vector::KnotVector;
use crate::real_knot_vector::{RealKnotVector, KnotConfig};

pub struct BSplineBasisConfig {
    pub start: f64,
    pub end: f64,
    pub n_basis: usize,
    pub order: usize
}

pub struct RealBSplineBasis {
    knot_vector: RealKnotVector,
    config: BSplineBasisConfig,
    degree: usize
}

impl BSplineBasis<f64> for RealBSplineBasis {
    type Config = BSplineBasisConfig;
    type KV = RealKnotVector;

    fn new(config: Self::Config) -> Self {

        let degree = config.order - 1;

        let knot_config = KnotConfig {
            n_knots: config.n_basis + config.order,
            multiplicity: config.order - 1,
            start: config.start,
            end: config.end
        };

        let knot_vector = RealKnotVector::build(knot_config);
       

        Self{knot_vector, config, degree}
    }

    fn b(&self, i: usize, x: f64) -> f64 {
        self.b_internal(i, x, self.degree)
    }

    fn db(&self, i: usize, x: f64) -> f64 {
        self.db_internal(i, x, self.degree)
    }

    fn get_knot_vector(&self) ->  &Self::KV {
        &self.knot_vector
    }

    fn get_n_basis(&self) -> usize {
        self.config.n_basis
    }
    
}