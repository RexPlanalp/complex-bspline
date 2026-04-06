use std::fmt;
pub enum BSplineError {
    InvalidNumberOfKnots { n_knots: usize, multiplicity: usize },
    InvalidKnotRange { start: f64, end: f64 },
}

pub type Result<T> = std::result::Result<T, BSplineError>;

impl fmt::Display for BSplineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BSplineError::InvalidNumberOfKnots {
                n_knots,
                multiplicity,
            } => {
                write!(
                    f,
                    "n_knots: {} should be greater than or equal to 2 * multiplicity: {}.",
                    n_knots,
                    2 * multiplicity
                )
            }
            BSplineError::InvalidKnotRange { start, end } => {
                write!(f, "Start: {} should be less than end: {}.", start, end)
            }
        }
    }
}
