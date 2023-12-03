pub fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 2..=n {
        result *= i;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        let samples = [
            (0, 1),
            (1, 1),
            (2, 2),
            (3, 6),
            (4, 24),
            (5, 120),
            (10, 3628800),
        ];

        for &(input, expected) in &samples {
            assert_eq!(factorial(input), expected);
        }
    }
}
