use crate::error::Result;
pub trait Config {
    fn validate(&self) -> Result<()>;
}

pub mod knots;
