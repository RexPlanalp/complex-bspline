use std::f64::consts::PI;

use bspline_basis::bsplines::dump::dump_basis;
use bspline_basis::bsplines::{
    complex::{ComplexBSplineBasis, ComplexBSplineBasisConfig},
    real::BSplineBasisConfig,
};
use bspline_basis::knots::complex::EcsConfig;
use bspline_basis::integrator::BSplineBasisIntegrator;
use bspline_basis::bsplines::basis::BSplineBasis;
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

    let integrator = BSplineBasisIntegrator::new(&complex_basis);
    let val =integrator.integrate(28, 28, |basis, i, j, x| {
    basis.b_at_scalar(i, x) * basis.b_at_scalar(j, x)
    });



    println!("{val}");

    dump_basis(&complex_basis, 1000).expect("Should dump");
}
