use complex_bspline::knots::{KnotVector, KnotVectorConfig};

fn main() {

    let knot_config = KnotVectorConfig{
        start: 0.0,
        end: 10.0,
        n: 50,
        r0: 5.0,
        eta: 0.25,
        multiplicity: 3
    };

    let knot_vector = KnotVector::new(knot_config);

    print!("{:#?}", knot_vector);
}