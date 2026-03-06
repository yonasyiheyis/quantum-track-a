use std::ops::Mul;

// Minimal local complex type for this exercise's offline math core.
// This is intentionally not a full complex-number implementation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Complex64 {
    re: f64,
    im: f64,
}

impl Complex64 {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    pub fn norm_sqr(self) -> f64 {
        self.re * self.re + self.im * self.im
    }
}

impl Mul<f64> for Complex64 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            re: self.re * rhs,
            im: self.im * rhs,
        }
    }
}

pub fn mag2(z: Complex64) -> f64 {
    // |z|^2 = a^2 + b^2
    z.norm_sqr()
}

pub fn normalize(vec: &[Complex64]) -> Result<Vec<Complex64>, &'static str> {
    let norm2: f64 = vec.iter().map(|z| z.norm_sqr()).sum();
    if norm2 == 0.0 {
        return Err("Cannot normalize the zero vector.");
    }
    let scale = 1.0 / norm2.sqrt();
    Ok(vec.iter().map(|z| *z * scale).collect())
}

pub fn probs(vec: &[Complex64]) -> Result<Vec<f64>, &'static str> {
    let v = normalize(vec)?;
    Ok(v.iter().map(|z| z.norm_sqr()).collect())
}
