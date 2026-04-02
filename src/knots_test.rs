use num_complex::ComplexFloat;
use std::io::{BufWriter, Write};
use std::fs::File;

pub trait KnotVector<T: ComplexFloat<Real = f64>> {
    type Config;

    // Public
    fn build(config: Self::Config) -> Self;
    
    fn get_knots(&self) -> &[T];

    fn dump(&self) -> std::io::Result<()> {
        let output_file = File::create("knots.txt")?;
        let mut writer = BufWriter::new(output_file);

        for x in self.get_knots() {
            writeln!(writer, "{} {}", x.re(), x.im())?;
        }

        Ok(())
    }

    fn in_interval(&self, x: f64, i: usize) -> bool {
        x >= self.get_knots()[i].re() && x < self.get_knots()[i + 1].re()
    }

    // Private
    fn validate_knot_config(n_knots: usize, multiplicity: usize, start: f64, end: f64) {
        assert!(
            n_knots >= 2 * multiplicity,
            "number of knots must be at least 2 * multiplicity"
        );
        assert!(start <= end, "start must be <= end");
    }

    fn build_linear_knots(n_knots: usize, multiplicity: usize, start: f64, end: f64) -> Vec<f64> {
        let n_middle = n_knots - 2 * multiplicity;
        let step = (end - start) / ((n_middle + 1) as f64);

        std::iter::repeat_n(start, multiplicity)
            .chain((1..=n_middle).map(|i| start + i as f64 * step))
            .chain(std::iter::repeat_n(end, multiplicity))
            .collect()
    }
}