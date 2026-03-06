pub mod qc_math;

#[cfg(test)]
mod tests {
    use crate::qc_math::{mag2, probs, Complex64};

    fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() <= tol
    }

    #[test]
    fn test_mag2_3_4i() {
        assert_eq!(mag2(Complex64::new(3.0, 4.0)), 25.0);
    }

    #[test]
    fn test_mag2_1_minus_i() {
        assert_eq!(mag2(Complex64::new(1.0, -1.0)), 2.0);
    }

    #[test]
    fn test_probs_equal_superposition() {
        let p = probs(&[Complex64::new(1.0, 0.0), Complex64::new(1.0, 0.0)]).unwrap();
        assert!(approx_eq(p[0], 0.5, 1e-12));
        assert!(approx_eq(p[1], 0.5, 1e-12));
    }

    #[test]
    fn test_probs_single_state() {
        let p = probs(&[Complex64::new(1.0, 1.0), Complex64::new(0.0, 0.0)]).unwrap();
        assert!(approx_eq(p[0], 1.0, 1e-12));
        assert!(approx_eq(p[1], 0.0, 1e-12));
    }

    #[test]
    fn test_probs_sum_to_one() {
        let p = probs(&[Complex64::new(1.0, 2.0), Complex64::new(3.0, -4.0)]).unwrap();
        let sum: f64 = p.iter().sum();
        assert!(approx_eq(sum, 1.0, 1e-12));
    }

    #[test]
    fn test_zero_vector_error() {
        let err = probs(&[Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0)]).unwrap_err();
        assert_eq!(err, "Cannot normalize the zero vector.");
    }
}
