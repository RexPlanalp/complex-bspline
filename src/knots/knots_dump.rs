use crate::core::dump::Dump;
use crate::core::knot_vector::KnotVector;
use std::fs::File;
use std::io::{BufWriter, Write};
use crate::core::scalar::BSplineScalar;
use std::path::Path;pub struct KnotsDump{}

impl Dump for KnotsDump{

    fn dump<K: KnotVector>(&self, kv: &K) -> std::io::Result<()> {
        let path = Path::new("output").join(kv.outfile());
        let output_file = File::create(path)?;
        let mut writer = BufWriter::new(output_file);

        for x in kv.knots() {
            writeln!(writer, "{} {}", x.real(), x.imag())?;
        }

        Ok(())
    }
}