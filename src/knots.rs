use num_complex::Complex64;

struct KnotVectorConfig{
    start: f64,
    end: f64,
    r0: f64,
    eta: f64,
    multiplicity: usize,
}

struct KnotVector{
    knots: Vec<Complex64>,
    config: KnotVectorConfig
}



impl KnotVector {
    fn new(config: KnotVectorConfig) -> Self {
        todo!()
    }
}