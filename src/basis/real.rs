use crate::config::basis::BasisConfig;
use crate::knots::real::RealKnotVector;
use crate::core::basis::BSplineBasis;

pub struct RealBSplineBasis {
    knot_vector: RealKnotVector,
    config: BasisConfig,
}

impl BSplineBasis for RealBSplineBasis {
    type KV = RealKnotVector;

    fn b(&self, i: usize, x: f64) -> <Self::KV as crate::core::knot_vector::KnotVector>::Scalar {
        crate::core::eval::b(i, x, self.knot_vector(), self.degree())
    }

    fn db(&self, i: usize, x: f64) -> <Self::KV as crate::core::knot_vector::KnotVector>::Scalar {
        crate::core::eval::db(i, x, self.knot_vector(), self.degree())
    }

    fn knot_vector(&self) -> &Self::KV {
        &self.knot_vector
    }

    fn n_basis(&self) -> usize {
        self.config.order
    }

    fn order(&self) -> usize {
        self.config.order
    }

    fn degree(&self) -> usize {
        self.config.order - 1
    }
}