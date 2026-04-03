use complex_bspline::real_knot_vector::{KnotConfig, RealKnotVector};
use complex_bspline::knot_vector::KnotVector;
use complex_bspline::real_bspline_basis::{BSplineBasisConfig, RealBSplineBasis};
use complex_bspline::complex_bspline_basis::{ComplexBSplineBasis, ComplexBSplineBasisConfig};
use complex_bspline::bspline_basis::BSplineBasis;
use complex_bspline::complex_knot_vector::{ComplexKnotConfig, ComplexKnotVector, EcsConfig};
use std::f64::consts::PI;

fn main() {
    
    // let bspline_basis_config = BSplineBasisConfig { start: 0.0, end: 10.0, n_basis: 30, order: 7 };
    // let real_basis = RealBSplineBasis::new(bspline_basis_config);
    let ecs_config = EcsConfig{
        r0: 5.0,
        eta: PI / 4.0
    };

    let bspline_basis_config = BSplineBasisConfig{
        start: 0.0,
        end: 10.0,
        n_basis: 30,
        order: 7
    };

    let complex_bspline_basis_config = ComplexBSplineBasisConfig {
        config: bspline_basis_config,
        ecs_config
    };

    let complex_basis = ComplexBSplineBasis::new(complex_bspline_basis_config);

    complex_basis.dump(0.01);
}
