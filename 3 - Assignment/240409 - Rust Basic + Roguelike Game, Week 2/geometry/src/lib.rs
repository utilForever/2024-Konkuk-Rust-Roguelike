

// Calculate the magnitude of a vector by summing the squares of its coordinates
// and taking the square root. Use the `sqrt()` method to calculate the square
// root, like `v.sqrt()`.
fn magnitude(vec:&[f64;3]) -> f64 {
    let pow = vec[0].powf(2.0) + vec[1].powf(2.0) + vec[2].powf(2.0);
    if pow < 0.0 {
        return -1.0;
    }
    return pow.sqrt();
}

// Normalize a vector by calculating its magnitude and dividing all of its
// coordinates by that magnitude.
fn normalize(vec:&mut [f64;3]) {
    let mag = magnitude(vec);
    vec[0] /= mag;
    vec[1] /= mag;
    vec[2] /= mag;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn magnitude_normal() {
        let mut vec = [1.0, 2.0, 9.0];
        assert!((magnitude(&mut vec) - 9.273618495495704).abs() < f64::EPSILON);
    }

    #[test]
    fn normalize_normal() {
        let mut vec = [1.0, 2.0, 9.0];
        normalize(&mut vec);
        assert!((magnitude(&mut vec) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn magnitude_unit() {
        let mut vec = [0.0, 1.0, 0.0];
        assert!((magnitude(&mut vec) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn normalize_unit() {
        let mut vec = [0.0, 1.0, 0.0];
        normalize(&mut vec);
        assert!((magnitude(&mut vec) - 1.0).abs() < f64::EPSILON);
    }
}
