
use num_complex::{ComplexFloat, Complex64};
use num_traits::{Zero, One};
pub trait BSplineScalar: ComplexFloat<Real = f64> + Zero + One {
    fn from_usize(n: usize) -> Self;
}

impl BSplineScalar for f64 {
    fn from_usize(n: usize) -> Self {
        n as f64
    }
}

impl BSplineScalar for Complex64 {
    fn from_usize(n: usize) -> Self {
        Complex64::new(n as f64, 0.0)
    }
}