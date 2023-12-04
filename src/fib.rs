pub fn fib1(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n <= 2 {
        1
    } else {
        fib1(n - 1) + fib1(n - 2)
    }
}

pub fn fib2(n: usize) -> usize {
    if n <= 1 {
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
        let samples = [
            (0, 0),
            (1, 1),
            (2, 1),
            (3, 2),
            (4, 3),
            (5, 5),
            (6, 8),
            (20, 6765),
        ];

        for &(input, expected) in &samples {
            assert_eq!(fib1(input), expected);
        }
    }

    #[test]
    fn test_fib2() {
        let samples = [
            (0, 0),
            (1, 1),
            (2, 1),
            (3, 2),
            (4, 3),
            (5, 5),
            (6, 8),
            (20, 6765),
        ];

        for &(input, expected) in &samples {
            assert_eq!(fib2(input), expected);
        }
    }
}
