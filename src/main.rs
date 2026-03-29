use complex_bspline::knots::{KnotVector, KnotVectorConfig};
use std::f64::consts::PI;

fn main() {

    let knot_config = KnotVectorConfig{
        start: 0.0,
        end: 10.0,
        n: 50,
        r0: 7.0,
        eta: PI*0.25,
        multiplicity: 3
    };

    let knot_vector = KnotVector::new(knot_config);

    print!("{:#?}", knot_vector);
}