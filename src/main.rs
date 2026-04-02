
use complex_bspline::knots_test::{KnotVector, KnotConfig, RealKnotVector, EcsConfig, ComplexKnotConfig, ComplexKnotVector};
use std::f64::consts::PI;

fn main() {
    let knot_config = KnotConfig {
        n_knots: 30,
        multiplicity: 4,
        start: 0.0,
        end: 10.0
    };

    let ecs_config = EcsConfig {
        r0: 5.0,
        eta: PI / 4.0
    };

    let complex_knot_config = ComplexKnotConfig {
        knot_config,
        ecs_config
    };

    let complex_knots = ComplexKnotVector::build(complex_knot_config);
    let _ = complex_knots.dump();

    println!("{:?}", complex_knots);
}
