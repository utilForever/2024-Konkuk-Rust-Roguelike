pub fn fib(n: u32) -> u32 {
    if n <= 2 {
        1
    } else {
        // The recursive case
        return fib(n-1) + fib(n-2);
    }
}

pub fn fib_iterative(n: u32) -> u32 {
    let target: usize = n as usize;
    let mut dp = vec![0; target+1];
    dp[0] = 0;
    dp[1] = 1;
    for x in 2..=target {
        dp[x] = dp[x-1] + dp[x-2];
    }
    dp[target]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_1() {
        let ret = fib(1);
        assert_eq!(ret, 1);
    }

    #[test]
    fn fib_2() {
        let ret = fib(2);
        assert_eq!(ret, 1);
    }

    #[test]
    fn fib_5() {
        let ret = fib(5);
        assert_eq!(ret, 5);
    }

    #[test]
    fn fib_20() {
        let ret = fib(20);
        assert_eq!(ret, 6765);
    }

    #[test]
    fn fib_iterative_5() {
        let ret = fib_iterative(5);
        assert_eq!(ret, 5)
    }

    #[test]
    fn fib_iterative_20() {
        let ret = fib_iterative(20);
        assert_eq!(ret, 6765);
    }
}
