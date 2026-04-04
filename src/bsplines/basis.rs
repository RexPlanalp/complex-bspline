use crate::knots::knot_vector::KnotVector;
use ndarray::linspace;
use crate::scalar::BSplineScalar;
use std::fs::File;
use std::io::{BufWriter, Write};
use crate::knots::dump::dump_knots;
pub trait BSplineBasis<T: BSplineScalar> {
    type Config;
    type KV: KnotVector<T>;

    fn new(config: Self::Config) -> Self;
    fn b(&self, i: usize, x: f64) -> T;
    fn db(&self, i: usize, x: f64) -> T;
    fn get_knot_vector(&self) -> &Self::KV;
    fn get_n_basis(&self) -> usize;

    fn dump(&self, samples: usize) -> std::io::Result<()> {
        dump_knots(self.get_knot_vector()).expect("Should dump");

        let output_file = File::create("output/B.txt")?;
        let mut writer = BufWriter::new(output_file);

        let x_range: Vec<f64> = linspace(self.get_knot_vector().get_start(), self.get_knot_vector().get_end(), samples).collect();

        for i in 0..self.get_n_basis() {
            for &x in &x_range {
                let eval = self.b(i, x);
                writeln!(writer, "{} {}", eval.re(), eval.im())?;
            }
        }

        let output_file = File::create("output/dB.txt")?;
        let mut writer = BufWriter::new(output_file);

        for i in 0..self.get_n_basis() {
            for &x in &x_range {
                let eval = self.db(i, x);
                writeln!(writer, "{} {}", eval.re(), eval.im())?;
            }
        }

        let metadata_file = File::create("output/basis_meta.txt")?;
        let mut writer = BufWriter::new(metadata_file);

        writeln!(writer, "{}", self.get_n_basis())?;
        for &x in &x_range {
            writeln!(writer, "{x}")?;
        }

        Ok(())
    }
}
