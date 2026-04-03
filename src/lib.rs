
mod util;

pub mod knot_vector;
pub mod real_knot_vector;
pub mod complex_knot_vector;

pub mod bspline_basis;
pub mod real_bspline_basis;
pub mod complex_bspline_basis;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
