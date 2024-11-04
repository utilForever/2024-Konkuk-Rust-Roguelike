fn check_cache(mut c: impl FnMut(i32) -> i32) -> impl FnMut(i32) -> (i32, bool) {
    let mut has_cache = false;
    let mut cached_in: i32 = 0;
    let mut cached_out: i32 = 0;

    move |x| {
        if has_cache && cached_in == x {
            (cached_out, true)
        } else {
            has_cache = true;
            cached_in = x;
            cached_out = c(x);
            (cached_out, false)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::check_cache;

    #[test]
    fn test_compute_and_cache() {
        let mut compute_func = check_cache(|x| x * x);

        assert_eq!(compute_func(4), (16, false)); // Compute: 16
        assert_eq!(compute_func(4), (16, true)); // Use cache: 16
        assert_eq!(compute_func(5), (25, false)); // Compute: 25
    }
}
