use crate::core::knot_vector::KnotVector;
use crate::core::scalar::BSplineScalar;
use crate::io::dump::Dump;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
pub struct KnotsDump {}

impl<K: KnotVector> Dump<K> for KnotsDump {
    fn dump(&self, kv: &K) -> std::io::Result<()> {
        let path = Path::new("output").join(kv.outfile());
        let output_file = File::create(path)?;
        let mut writer = BufWriter::new(output_file);

        for x in kv.knots() {
            writeln!(writer, "{} {}", x.real(), x.imag())?;
        }

        Ok(())
    }
}
