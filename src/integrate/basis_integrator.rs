use crate::core::basis::BSplineBasis;
use crate::core::knot_vector::KnotVector;
use crate::core::scalar::BSplineScalar;
use num_traits::Zero;
use std::cmp::{min, max};

pub struct BSplineBasisIntegrator<'a, B: BSplineBasis> {
    basis: &'a B,
    roots: &'static [f64],
    weights: &'static [f64],
}

impl<'a, B: BSplineBasis> BSplineBasisIntegrator<'a, B> {
    pub fn integrate<F>(
        &self,
        i: usize,
        j: usize,
        f: F
    ) -> <<B as BSplineBasis>::KV as KnotVector>::Scalar 
    where 
        F: Fn(&B, usize, usize, <B::KV as KnotVector>::Scalar) -> <B::KV as KnotVector>::Scalar,
    {
        let mut result = <<B as BSplineBasis>::KV as KnotVector>::Scalar::zero();
        
        let half = <<B as BSplineBasis>::KV as KnotVector>::Scalar::from_f64(0.5);

        let lower = min(i, j);
        let upper = max(i, j);

        for k in lower..=upper + self.basis.degree() {
            let (a,b) = self.basis.knot_vector().interval(k);

            if a == b {
                continue;
            }

            let half_b_minus_a = half * (b - a);
            let half_b_plus_a = half * (b + a);

            for (&r, &w) in self.roots.iter().zip(self.weights.iter()) {
                let r = <<B as BSplineBasis>::KV as KnotVector>::Scalar::from_f64(r);
                let w = <<B as BSplineBasis>::KV as KnotVector>::Scalar::from_f64(w) * half_b_minus_a;
                let x = half_b_minus_a * r + half_b_plus_a;
                
                let integrand_val = f(self.basis, i, j, x);
                
                result = result + w * integrand_val;
            }
        }

        result
    }

    pub fn new(basis: &'a B) -> Self {
        let (roots, weights) = Self::find_params(basis.order());

        Self {basis, roots, weights}
    }

    fn find_params(order: usize) -> (&'static[f64], &'static[f64]) {
        match order {
            2 => (&[-0.57735027, 0.57735027],&[1.0, 1.0]),
            3 => (&[-0.77459667, 0.0, 0.77459667], &[0.55555556, 0.88888889, 0.55555556]),
            4 => (&[-0.86113631, -0.33998104, 0.33998104, 0.86113631], &[0.34785485, 0.65214515, 0.65214515, 0.34785485]),
            5 => (&[-0.90617985, -0.53846931, 0.0, 0.53846931, 0.90617985], &[0.23692689, 0.47862867, 0.56888889, 0.47862867, 0.23692689]),
            6 => (&[-0.93246951, -0.66120939, -0.23861919, 0.23861919, 0.66120939, 0.93246951], &[0.17132449, 0.36076157, 0.46791393, 0.46791393, 0.36076157, 0.17132449]),
            7 => (&[-0.94910791, -0.74153119, -0.40584515, 0.0, 0.40584515, 0.74153119, 0.94910791], &[0.12948497, 0.27970539, 0.38183005, 0.41795918, 0.38183005, 0.27970539, 0.12948497]),
            8 => (&[-0.96028986, -0.79666648, -0.52553241, -0.18343464, 0.18343464, 0.52553241, 0.79666648, 0.96028986], &[0.10122854, 0.22238103, 0.31370665, 0.36268378, 0.36268378, 0.31370665, 0.22238103, 0.10122854]),
            _ => panic!("Order not supported for BSplineBasisIntegrator")
        }
    }
}