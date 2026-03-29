use crate::knots::{KnotVector, KnotVectorConfig};

struct BSpline {
    order: usize,
    knot_vector: KnotVector
}