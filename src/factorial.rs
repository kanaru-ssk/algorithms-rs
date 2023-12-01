pub fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 2..(n + 1) {
        result *= i;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_correct_value() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(10), 3628800);
    }
}
