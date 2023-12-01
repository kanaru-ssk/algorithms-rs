use super::gcd::gcd;

pub fn lcm(a: u32, b: u32) -> u32 {
    if a == 0 && b == 0 {
        0
    } else {
        a / gcd(a, b) * b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_correct_value() {
        assert_eq!(lcm(18, 12), 36);
        assert_eq!(lcm(12, 18), 36);
        assert_eq!(lcm(1, 1), 1);
        assert_eq!(lcm(10, 0), 0);
        assert_eq!(lcm(0, 10), 0);
        assert_eq!(lcm(0, 0), 0);
    }
}
