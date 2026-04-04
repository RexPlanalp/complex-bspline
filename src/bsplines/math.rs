use crate::{knots::knot_vector::KnotVector, scalar::BSplineScalar};


pub fn b_internal<T, K>(i: usize, x: T, knot_vector: &K, degree: usize) -> T 
where
    T: BSplineScalar,
    K: KnotVector<T>
    {
        if degree == 0 {
            return if knot_vector.in_interval(x, i) {
                T::one()
            } else {
                T::zero()
            };
        }

        let knots = knot_vector.get_knots();

        let denom1 = knots[i + degree] - knots[i];
        let denom2 = knots[i + degree + 1] - knots[i + 1];

        let term1 = if denom1.abs() != 0.0 {
            (x - knots[i]) / (denom1)
                * b_internal(i, x, knot_vector, degree - 1)
        } else {
            T::zero()
        };

        let term2 = if denom2.abs() != 0.0 {
            (knots[i + degree + 1] - x) / (denom2)
                * b_internal(i + 1, x, knot_vector, degree - 1)
        } else {
            T::zero()
        };

        return term1 + term2;
    }

pub fn db_internal<T, K>(i: usize, x: T, knot_vector: &K, degree: usize) -> T 
where 
    T: BSplineScalar,
    K: KnotVector<T>
    {
        if degree == 0 {
            return T::zero();
        }

        let knots = knot_vector.get_knots();

        let denom1 =
            knots[i + degree] - knots[i];
        let denom2 = knots[i + degree + 1] - knots[i + 1];

        let term1 = if denom1.abs() != 0.0 {
            <T as BSplineScalar>::from_usize(degree) / denom1 * b_internal(i, x, knot_vector,degree - 1)
        } else {
            T::zero()
        };

        let term2 = if denom2.abs() > 0.0 {
            <T as BSplineScalar>::from_usize(degree) / denom2 * b_internal(i + 1, x, knot_vector,degree - 1)
        } else {
            T::zero()
        };

        term1 - term2
    }