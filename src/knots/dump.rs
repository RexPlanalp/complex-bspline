use crate::knots::knot_vector::KnotVector;
use num_complex::ComplexFloat;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

pub fn dump_knots<K: KnotVector>(kv: &K) -> std::io::Result<()> {
    let path = Path::new("output").join(kv.get_outfile());
    let output_file = File::create(path)?;
    let mut writer = BufWriter::new(output_file);

    for x in kv.get_knots() {
        writeln!(writer, "{} {}", x.re(), x.im())?;
    }

    Ok(())
}
