pub fn fib1(n: u32) -> u32 {
    if n <= 2 {
        return 1;
    }

    fib1(n - 1) + fib1(n - 2)
}

pub fn fib2(n: usize) -> usize {
    if n == 0 || n == 1 {
        return n;
    }

    let mut a: Vec<usize> = vec![0; n];

    a[0] = 1;
    a[1] = 1;

    for i in 2..n {
        a[i] = a[i - 1] + a[i - 2];
    }

    a[n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib1() {
        assert_eq!(fib1(1), 1);
        assert_eq!(fib1(6), 8);
        assert_eq!(fib1(0), 1);
    }

    #[test]
    fn test_fib2() {
        assert_eq!(fib2(1), 1);
        assert_eq!(fib2(6), 8);
        assert_eq!(fib2(0), 0);
    }
}
