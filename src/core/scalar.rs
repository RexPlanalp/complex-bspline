use num_complex::Complex64;
use num_traits::{One, Zero};
use core::ops::{Add, Div, Mul, Sub};

pub trait BSplineScalar:
    Copy
    + Zero
    + One
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{
    fn from_usize(n: usize) -> Self;
    fn from_f64(f: f64) -> Self;
    fn real(self) -> f64;
    fn imag(self) -> f64;
    fn abs_f64(self) -> f64;
}

impl BSplineScalar for f64 {
    fn from_usize(n: usize) -> Self {
        n as f64
    }
    fn from_f64(f: f64) -> Self {
        f
    }
    fn real(self) -> f64 {
        self
    }
    fn imag(self) -> f64 {
        0.0
    }
    fn abs_f64(self) -> f64 {
        self.abs()
    }
}

impl BSplineScalar for Complex64 {
    fn from_usize(n: usize) -> Self {
        Complex64::new(n as f64, 0.0)
    }
    fn from_f64(f: f64) -> Self {
        Complex64::new(f, 0.0)
    }
    fn real(self) -> f64 {
        self.re
    }
    fn imag(self) -> f64 {
        self.im
    }
    fn abs_f64(self) -> f64 {
        self.norm()
    }
}
