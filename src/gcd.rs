pub fn gcd(a: u32, b: u32) -> u32 {
    // 割り切れるまで再帰的に呼び出す
    if b != 0 {
        return gcd(b, a % b);
    }

    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_correct_value() {
        assert_eq!(gcd(18, 12), 6);
        assert_eq!(gcd(12, 18), 6);
        assert_eq!(gcd(1, 1), 1);
        assert_eq!(gcd(10, 0), 10);
        assert_eq!(gcd(0, 10), 10);
        assert_eq!(gcd(0, 0), 0);
    }
}
