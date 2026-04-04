use std::f64::consts::PI;

use complex_bspline::knots::complex::EcsConfig;
use complex_bspline::bsplines::{basis::BSplineBasis, real::{ BSplineBasisConfig}, complex::{ComplexBSplineBasis, ComplexBSplineBasisConfig}};

fn main() {
    let ecs_config = EcsConfig {
        r0: 5.0,
        eta: PI / 4.0,
    };

    let bspline_basis_config = BSplineBasisConfig {
        start: 0.0,
        end: 10.0,
        n_basis: 30,
        order: 7,
    };

    let complex_bspline_basis_config = ComplexBSplineBasisConfig {
        config: bspline_basis_config,
        ecs_config,
    };

    let complex_basis = ComplexBSplineBasis::new(complex_bspline_basis_config);

    complex_basis.dump(0.01).expect("Should plot right now");
}
