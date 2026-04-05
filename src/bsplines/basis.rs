use crate::knots::knot_vector::KnotVector;
use crate::bsplines::math::{b_internal, db_internal};

pub trait BSplineBasis {
    type Config;
    type KV: KnotVector;

    fn b(&self, i: usize, x: f64) -> <Self::KV as KnotVector>::Scalar;
    fn db(&self, i: usize, x: f64) -> <Self::KV as KnotVector>::Scalar;
    fn knot_vector(&self) -> &Self::KV;
    fn n_basis(&self) -> usize;
    fn order(&self) -> usize;
    fn degree(&self) -> usize;

    fn b_at_scalar(&self, i: usize, x: <Self::KV as KnotVector>::Scalar)
            -> <Self::KV as KnotVector>::Scalar {
        b_internal(i, x, self.knot_vector(), self.degree())
    }

    fn db_at_scalar(&self, i: usize, x: <Self::KV as KnotVector>::Scalar)
            -> <Self::KV as KnotVector>::Scalar {
        db_internal(i, x, self.knot_vector(), self.degree())
    }
}
