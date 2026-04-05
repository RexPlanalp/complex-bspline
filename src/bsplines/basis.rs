use crate::knots::knot_vector::KnotVector;
use crate::scalar::BSplineScalar;

pub trait BSplineBasis<T: BSplineScalar> {
    type Config;
    type KV: KnotVector;

    fn new(config: Self::Config) -> Self;
    fn b(&self, i: usize, x: f64) -> T;
    fn db(&self, i: usize, x: f64) -> T;
    fn get_knot_vector(&self) -> &Self::KV;
    fn get_n_basis(&self) -> usize;
}
