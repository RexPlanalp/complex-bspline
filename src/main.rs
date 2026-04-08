use bspline_basis::config::basis::BasisConfig;
use bspline_basis::basis::complex::ComplexBSplineBasis;
use bspline_basis::basis::dump_basis::BasisDump;
use bspline_basis::core::dump::Dump;
use bspline_basis::config::ecs::EcsConfig;
use std::f64::consts::PI;

fn main() {
    let basis_config = BasisConfig {
        n_basis: 30,
        order: 7,
        start: 0.0,
        end: 10.0
    };

    let ecs_config = EcsConfig {
        r0: 5.0,
        eta: PI/4.0
    };


    let complex_basis = ComplexBSplineBasis::try_new(basis_config, ecs_config).expect("Failed to construct BSpline Basis");

    let basis_dumper = BasisDump{samples: 1000};

    basis_dumper.dump(&complex_basis).expect("Error Dumping BSpline Basis");
}