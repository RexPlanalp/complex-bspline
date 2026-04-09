use bspline_basis::basis::complex::ComplexBSplineBasis;
use bspline_basis::config::basis::BasisConfig;
use bspline_basis::config::ecs::EcsConfig;
use bspline_basis::core::basis::BSplineBasis;
use bspline_basis::io::dump::Dump;
use bspline_basis::io::dump_basis::BasisDump;
use std::f64::consts::PI;
use bspline_basis::integrate::basis_integrator::BSplineBasisIntegrator;
fn main() {
    let basis_config = BasisConfig {
        n_basis: 30,
        order: 7,
        start: 0.0,
        end: 10.0,
    };

    let ecs_config = EcsConfig {
        r0: 5.0,
        eta: PI / 4.0,
    };

    let complex_basis = ComplexBSplineBasis::try_new(basis_config, ecs_config)
        .expect("Failed to construct BSpline Basis");

    let basis_dumper = BasisDump { samples: 1000 };

    basis_dumper
        .dump(&complex_basis)
        .expect("Failed to dump BSpline Basis");

    let integrator = BSplineBasisIntegrator::new(&complex_basis);

    let val = integrator.integrate(28, 28, |basis, i, j, x| {
    basis.eval_b(i, x) * basis.eval_b(j, x)
    });

    println!("{val}");


}
