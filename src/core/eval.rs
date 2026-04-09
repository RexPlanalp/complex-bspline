use crate::core::knot_vector::KnotVector;
use crate::core::scalar::BSplineScalar;
use num_traits::{One, Zero};

pub fn b<K>(i: usize, x: K::Scalar, knot_vector: &K, degree: usize) -> K::Scalar
where
    K: KnotVector,
{
    if degree == 0 {
        return if knot_vector.in_interval(x, i) {
            K::Scalar::one()
        } else {
            K::Scalar::zero()
        };
    }

    let knots = knot_vector.knots();

    let denom1 = knots[i + degree] - knots[i];
    let denom2 = knots[i + degree + 1] - knots[i + 1];

    let term1 = if denom1.abs_f64() != 0.0 {
        (x - knots[i]) / denom1 * b(i, x, knot_vector, degree - 1)
    } else {
        K::Scalar::zero()
    };

    let term2 = if denom2.abs_f64() != 0.0 {
        (knots[i + degree + 1] - x) / denom2 * b(i + 1, x, knot_vector, degree - 1)
    } else {
        K::Scalar::zero()
    };

    term1 + term2
}

pub fn db<K>(i: usize, x: K::Scalar, knot_vector: &K, degree: usize) -> K::Scalar
where
    K: KnotVector,
{
    if degree == 0 {
        return K::Scalar::zero();
    }

    let knots = knot_vector.knots();

    let denom1 = knots[i + degree] - knots[i];
    let denom2 = knots[i + degree + 1] - knots[i + 1];

    let term1 = if denom1.abs_f64() != 0.0 {
        K::Scalar::from_usize(degree) / denom1 * b(i, x, knot_vector, degree - 1)
    } else {
        K::Scalar::zero()
    };

    let term2 = if denom2.abs_f64() != 0.0 {
        K::Scalar::from_usize(degree) / denom2 * b(i + 1, x, knot_vector, degree - 1)
    } else {
        K::Scalar::zero()
    };

    term1 - term2
}
