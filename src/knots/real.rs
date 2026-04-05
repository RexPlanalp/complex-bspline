use crate::knots::knot_vector::KnotVector;
pub struct KnotConfig {
    pub n_knots: usize,
    pub multiplicity: usize,
    pub start: f64,
    pub end: f64,
}
pub struct RealKnotVector {
    knots: Vec<f64>,
    config: KnotConfig,
}

impl KnotVector for RealKnotVector {
    type Scalar = f64;
    type Config = KnotConfig;

    fn build(config: Self::Config) -> Self {
        let knots = Self::build_linear_knots(
            config.n_knots,
            config.multiplicity,
            config.start,
            config.end,
        );

        Self { knots, config }
    }

    fn get_knots(&self) -> &[f64] {
        &self.knots
    }

    fn get_start(&self) -> f64 {
        self.config.start
    }

    fn get_end(&self) -> f64 {
        self.config.end
    }
}
