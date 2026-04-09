use crate::core::knot_vector::KnotVector;

pub trait BSplineBasis {
    type KV: KnotVector;

    fn eval_b(&self, i: usize, x: <Self::KV as KnotVector>::Scalar) -> <Self::KV as KnotVector>::Scalar {
        crate::core::eval::b(i, x, self.knot_vector(), self.degree())
    }

    fn eval_db(&self, i: usize, x: <Self::KV as KnotVector>::Scalar) -> <Self::KV as KnotVector>::Scalar {
        crate::core::eval::db(i, x, self.knot_vector(), self.degree())
    }

    fn b(&self, i: usize, x: f64) -> <Self::KV as KnotVector>::Scalar;
    fn db(&self, i: usize, x: f64) -> <Self::KV as KnotVector>::Scalar;
    fn knot_vector(&self) -> &Self::KV;
    fn n_basis(&self) -> usize;
    fn order(&self) -> usize;
    fn degree(&self) -> usize;
}
