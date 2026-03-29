use crate::knots::{KnotVector, KnotVectorConfig};
use crate::util::arange;
use crate::util::ecs_x;
use num_complex::{Complex64, ComplexFloat};
use std::fs::File;
use std::io::{BufWriter, Write};
pub struct BSpline {
    n: usize,
    order: usize,
    degree: usize,
    knot_vector: KnotVector,
}

impl BSpline {
    pub fn new(start: f64, end: f64, n: usize, order: usize, r0: f64, eta: f64) -> Self {
        let knot_vector_config = KnotVectorConfig {
            start,
            end,
            n: n + order,
            r0,
            eta,
            multiplicity: order - 1,
        };

        let knot_vector = KnotVector::new(knot_vector_config);

        Self {
            n,
            order,
            degree: order - 1,
            knot_vector,
        }
    }

    pub fn b(&self, i: usize, x: f64) -> Complex64 {
        let complex_x = ecs_x(x, self.knot_vector.config.r0, self.knot_vector.config.eta);
        self.b_recursive(i, complex_x, self.degree)
    }

    pub fn dump_b(&self, resolution: f64) -> std::io::Result<()> {
        let output_file = File::create("B.txt")?;
        let mut writer = BufWriter::new(output_file);

        let x_range = arange(
            self.knot_vector.config.start,
            self.knot_vector.config.end,
            resolution,
        );

        for i in 0..self.n {
            for &x in &x_range {
                let eval = self.b(i, x);
                writeln!(writer, "{} {}", eval.re, eval.im)?;
            }
        }

        let metadata_file = File::create("B_meta.txt")?;
        let mut writer = BufWriter::new(metadata_file);

        writeln!(writer, "{}", self.n)?;
        for &x in &x_range {
            writeln!(writer, "{x}")?;
        }

        Ok(())
    }

    fn b_recursive(&self, i: usize, x: Complex64, degree: usize) -> Complex64 {
        if degree == 0 {
            return if self.knot_vector.in_interval(x.re, i) {
                Complex64::new(1.0, 0.0)
            } else {
                Complex64::new(0.0, 0.0)
            };
        }

        let denom1 = self.knot_vector[i + degree] - self.knot_vector[i];
        let denom2 = self.knot_vector[i + degree + 1] - self.knot_vector[i + 1];

        let mut term1 = Complex64::new(0.0, 0.0);
        let mut term2 = Complex64::new(0.0, 0.0);

        if denom1.abs() != 0.0 {
            term1 = (x - self.knot_vector[i]) / (denom1) * self.b_recursive(i, x, degree - 1);
        }
        if denom2.abs() != 0.0 {
            term2 = (self.knot_vector[i + degree + 1] - x) / (denom2)
                * self.b_recursive(i + 1, x, degree - 1);
        }

        return term1 + term2;
    }

    pub fn db(&self, i: usize, x: f64) -> Complex64 {
        if (i == 0) || (i == self.n - 1) {
            return Complex64::from(0.0);
        }
        let complex_x = ecs_x(x, self.knot_vector.config.r0, self.knot_vector.config.eta);
        self.db_recursive(i, complex_x, self.degree)
    }

    pub fn dump_db(&self, resolution: f64) -> std::io::Result<()> {
        let output_file = File::create("dB.txt")?;
        let mut writer = BufWriter::new(output_file);

        let x_range = arange(
            self.knot_vector.config.start,
            self.knot_vector.config.end,
            resolution,
        );

        for i in 0..self.n {
            for &x in &x_range {
                let eval = self.db(i, x);
                writeln!(writer, "{} {}", eval.re, eval.im)?;
            }
        }

        let metadata_file = File::create("dB_meta.txt")?;
        let mut writer = BufWriter::new(metadata_file);

        writeln!(writer, "{}", self.n)?;
        for &x in &x_range {
            writeln!(writer, "{x}")?;
        }

        Ok(())
    }

    fn db_recursive(&self, i: usize, x: Complex64, degree: usize) -> Complex64 {
        if degree == 0 {
            return Complex64::from(0.0);
        }

        let denom1 = self.knot_vector[i + degree] - self.knot_vector[i];
        let denom2 = self.knot_vector[i + degree + 1] - self.knot_vector[i + 1];

        let mut term1 = Complex64::from(0.0);
        let mut term2 = Complex64::from(0.0);

        if denom1.abs() != 0.0 {
            term1 = Complex64::from(degree as f64) / denom1 * self.b_recursive(i, x, degree - 1);
        }
        if denom2.abs() != 0.0 {
            term2 =
                Complex64::from(degree as f64) / denom2 * self.b_recursive(i + 1, x, degree - 1);
        }

        term1 - term2
    }
}
