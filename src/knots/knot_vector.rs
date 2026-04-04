use crate::scalar::BSplineScalar;

pub trait KnotVector<T: BSplineScalar> {
    type Config;

    fn build(config: Self::Config) -> Self;

    fn get_knots(&self) -> &[T];

    fn get_outfile(&self) -> &'static str {
        "knots.txt"
    }

    fn in_interval(&self, x: T, i: usize) -> bool {
        x.re() >= self.get_knots()[i].re() && x.re() < self.get_knots()[i + 1].re()
    }

    fn validate_knot_config(n_knots: usize, multiplicity: usize, start: f64, end: f64) {
        assert!(
            n_knots >= 2 * multiplicity,
            "number of knots must be at least 2 * multiplicity"
        );
        assert!(start <= end, "start must be <= end");
    }

    fn build_linear_knots(n_knots: usize, multiplicity: usize, start: f64, end: f64) -> Vec<f64> {
        Self::validate_knot_config(n_knots, multiplicity, start, end);
        let n_middle = n_knots - 2 * multiplicity;
        let step = (end - start) / ((n_middle + 1) as f64);

        std::iter::repeat_n(start, multiplicity)
            .chain((1..=n_middle).map(|i| start + i as f64 * step))
            .chain(std::iter::repeat_n(end, multiplicity))
            .collect()
    }

    fn get_start(&self) -> f64;
    fn get_end(&self) -> f64;
}
