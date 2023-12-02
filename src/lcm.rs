use super::gcd::gcd;

pub fn lcm(a: u32, b: u32) -> u32 {
    if a == 0 && b == 0 {
        return 0;
    }

    a / gcd(a, b) * b
}

pub fn lcm_vec(vec: &Vec<u32>) -> u32 {
    let mut result = match vec.get(0) {
        Some(i) => *i,
        None => 0,
    };

    for num in vec {
        result = lcm(result, *num);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(18, 12), 36);
        assert_eq!(lcm(12, 18), 36);
        assert_eq!(lcm(1, 1), 1);
        assert_eq!(lcm(10, 0), 0);
        assert_eq!(lcm(0, 10), 0);
        assert_eq!(lcm(0, 0), 0);
    }

    #[test]
    fn test_lcm_vec() {
        assert_eq!(lcm_vec(&vec![18, 12]), 36);
        assert_eq!(lcm_vec(&vec![36, 12, 48, 120]), 720);
        assert_eq!(lcm_vec(&vec![36, 12, 48, 0]), 0);
        assert_eq!(lcm_vec(&vec![0, 0, 0]), 0);
        assert_eq!(lcm_vec(&vec![3]), 3);
        assert_eq!(lcm_vec(&vec![]), 0);
    }
}
