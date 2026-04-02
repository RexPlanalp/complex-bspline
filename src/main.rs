
use complex_bspline::knots_test::{KnotVector, KnotConfig, RealKnotVector};
use std::f64::consts::PI;

fn main() {
    let knot_config = KnotConfig {
        n_knots: 30,
        multiplicity: 4,
        start: 0.0,
        end: 10.0
    };

    let real_knots = RealKnotVector::build(knot_config);

    println!("{:?}", real_knots);
}
