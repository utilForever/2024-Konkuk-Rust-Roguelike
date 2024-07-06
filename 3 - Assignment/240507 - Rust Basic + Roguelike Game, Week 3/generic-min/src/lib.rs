use std::cmp::Ordering;

// TODO: implement the `min` function.
fn min<T: Ord>(v1: T, v2: T) -> T {
    if v1.cmp(&v2) == Ordering::Less {
        v1
    } else {
        v2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_number() {
        assert_eq!(min(0, 10), 0);
        assert_eq!(min(500, 123), 123);
    }

    #[test]
    fn min_char() {
        assert_eq!(min('a', 'z'), 'a');
        assert_eq!(min('7', '1'), '1');
    }

    #[test]
    fn min_string() {
        assert_eq!(min("hello", "goodbye"), "goodbye");
        assert_eq!(min("bat", "armadillo"), "armadillo");
    }
}
