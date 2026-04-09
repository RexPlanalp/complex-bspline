use crate::core::knot_vector::KnotVector;

pub trait BSplineBasis {
    type KV: KnotVector;

    fn b(&self, i: usize, x: f64) -> <Self::KV as KnotVector>::Scalar;
    fn db(&self, i: usize, x: f64) -> <Self::KV as KnotVector>::Scalar;
    fn knot_vector(&self) -> &Self::KV;
    fn n_basis(&self) -> usize;
    fn order(&self) -> usize;
    fn degree(&self) -> usize;
}
