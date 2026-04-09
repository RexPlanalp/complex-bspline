use crate::core::basis::BSplineBasis;
use crate::core::knot_vector::KnotVector;
use crate::core::scalar::BSplineScalar;
use crate::io::dump::Dump;
use crate::io::dump_knots::KnotsDump;
use ndarray::linspace;
use std::fs::File;
use std::io::{BufWriter, Write};
pub struct BasisDump {
    pub samples: usize,
}

impl<B: BSplineBasis> Dump<B> for BasisDump {
    fn dump(&self, obj: &B) -> std::io::Result<()> {
        let knots_dumper = KnotsDump {};
        knots_dumper.dump(obj.knot_vector())?;

        let output_file = File::create("output/B.txt")?;
        let mut writer = BufWriter::new(output_file);

        let x_range: Vec<f64> = linspace(
            obj.knot_vector().start(),
            obj.knot_vector().end(),
            self.samples,
        )
        .collect();

        for i in 0..obj.n_basis() {
            for &x in &x_range {
                let val = obj.b(i, x);
                writeln!(writer, "{} {}", val.real(), val.imag())?;
            }
        }

        let output_file = File::create("output/dB.txt")?;
        let mut writer = BufWriter::new(output_file);

        for i in 0..obj.n_basis() {
            for &x in &x_range {
                let val = obj.db(i, x);
                writeln!(writer, "{} {}", val.real(), val.imag())?;
            }
        }

        let metadata_file = File::create("output/basis_meta.txt")?;
        let mut writer = BufWriter::new(metadata_file);

        writeln!(writer, "{}", obj.n_basis())?;
        for &x in &x_range {
            writeln!(writer, "{x}")?;
        }

        Ok(())
    }
}
