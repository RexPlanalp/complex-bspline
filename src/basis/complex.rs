use crate::config::basis::BasisConfig;
use crate::config::ecs::EcsConfig;
use crate::config::knots::KnotConfig;
use crate::core::basis::BSplineBasis;
use crate::error::Result;
use crate::knots::complex::ComplexKnotVector;
use crate::transform::ecs::ecs_x;

pub struct ComplexBSplineBasis {
    knot_vector: ComplexKnotVector,
    config: BasisConfig,
}

impl BSplineBasis for ComplexBSplineBasis {
    type KV = ComplexKnotVector;

    fn b(&self, i: usize, x: f64) -> <Self::KV as crate::core::knot_vector::KnotVector>::Scalar {
        let x = ecs_x(
            x,
            self.knot_vector.ecs_config.r0,
            self.knot_vector.ecs_config.eta,
        );
        crate::core::eval::b(i, x, self.knot_vector(), self.degree())
    }

    fn db(&self, i: usize, x: f64) -> <Self::KV as crate::core::knot_vector::KnotVector>::Scalar {
        let x = ecs_x(
            x,
            self.knot_vector.ecs_config.r0,
            self.knot_vector.ecs_config.eta,
        );
        crate::core::eval::db(i, x, self.knot_vector(), self.degree())
    }

    fn knot_vector(&self) -> &Self::KV {
        &self.knot_vector
    }

    fn n_basis(&self) -> usize {
        self.config.n_basis
    }

    fn order(&self) -> usize {
        self.config.order
    }

    fn degree(&self) -> usize {
        self.config.order - 1
    }
}

impl ComplexBSplineBasis {
    pub fn try_new(config: BasisConfig, ecs_config: EcsConfig) -> Result<Self> {
        let knot_config = KnotConfig {
            n_knots: config.n_basis + config.order,
            multiplicity: config.order - 1,
            start: config.start,
            end: config.end,
        };

        let knot_vector = ComplexKnotVector::try_new(knot_config, ecs_config)?;

        Ok(Self {
            knot_vector,
            config,
        })
    }
}
