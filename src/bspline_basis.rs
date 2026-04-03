use num_complex::ComplexFloat;
use num_traits::{Zero, One, FromPrimitive};
use crate::knot_vector::KnotVector;
use std::fs::File;
use std::io::{BufWriter, Write};
use crate::util::arange;
pub trait BSplineBasis<T: ComplexFloat<Real = f64> + Zero + One + FromPrimitive> {
    type Config;
    type KV: KnotVector<T>;

    fn new(config: Self::Config) -> Self;
    fn b(&self, i:usize, x: f64) -> T;
    fn db(&self, i: usize, x: f64) -> T; 
    fn get_knot_vector(&self) -> &Self::KV;
    fn get_n_basis(&self) -> usize;

    fn b_internal(&self, i: usize, x: T, degree: usize) -> T {
        if degree == 0 {
            return if self.get_knot_vector().in_interval(x.re(), i) {
                T::one()
            } else {
                T::zero()
            };
        }

  

        let denom1 = self.get_knot_vector().get_knots()[i + degree] - self.get_knot_vector().get_knots()[i];
        let denom2 = self.get_knot_vector().get_knots()[i + degree + 1] - self.get_knot_vector().get_knots()[i + 1];

        let term1 = if denom1.abs() > 0.0 {
            (x - self.get_knot_vector().get_knots()[i]) / (denom1) * self.b_internal(i, x, degree - 1)
        } else {
            T::zero()
        };

        let term2 = if denom2.abs() > 0.0 {
            (self.get_knot_vector().get_knots()[i + degree + 1] - x) / (denom2)
                * self.b_internal(i + 1, x, degree - 1)
        } else {
            T::zero()
        };

        return term1 + term2;
    }

    fn db_internal(&self, i: usize, x: T, degree: usize) -> T {
        if degree == 0 {
            return T::zero()
        }

        let denom1 = self.get_knot_vector().get_knots()[i + degree] - self.get_knot_vector().get_knots()[i];
        let denom2 = self.get_knot_vector().get_knots()[i + degree + 1] - self.get_knot_vector().get_knots()[i + 1];


        let term1 = if denom1.abs() > 0.0 {
            T::from_usize(degree).unwrap() / denom1 * self.b_internal(i, x, degree - 1)
        } else {
            T::zero()
        };

        let term2 = if denom2.abs() > 0.0 {
            T::from_usize(degree).unwrap() / denom2 * self.b_internal(i + 1, x, degree - 1)
        } else {
            T::zero()
        };

        term1 - term2
    }

    fn dump(&self, resolution: f64) -> std::io::Result<()> {
        let output_file = File::create("output/B.txt")?;
        let mut writer = BufWriter::new(output_file);

        let x_range = arange(
            self.get_knot_vector().get_start(),
            self.get_knot_vector().get_end(),
            resolution,
            true,
        );

        for i in 0..self.get_n_basis() {
            for &x in &x_range {
                let eval = self.b(i, x);
                writeln!(writer, "{} {}", eval.re(), eval.im())?;
            }
        }

        let metadata_file = File::create("output/B_meta.txt")?;
        let mut writer = BufWriter::new(metadata_file);

        writeln!(writer, "{}", self.get_n_basis())?;
        for &x in &x_range {
            writeln!(writer, "{x}")?;
        }

        Ok(())
    }
}