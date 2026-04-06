use crate::{
    config::{Config, Result},
    error::BSplineError,
};
pub struct KnotConfig {
    pub n_knots: usize,
    pub multiplicity: usize,
    pub start: f64,
    pub end: f64,
}

impl Config for KnotConfig {
    fn validate(&self) -> Result<()> {
        if !(self.n_knots >= 2 * self.multiplicity) {
            return Err(BSplineError::InvalidNumberOfKnots {
                n_knots: self.n_knots,
                multiplicity: self.multiplicity,
            });
        }
        if !(self.start > self.end) {
            return Err(BSplineError::InvalidKnotRange {
                start: self.start,
                end: self.end,
            });
        }

        Ok(())
    }
}
