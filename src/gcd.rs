pub fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    }

    // 割り切れるまで再帰的に呼び出す
    gcd(b, a % b)
}

pub fn gcd_vec(vec: &Vec<u32>) -> u32 {
    let mut result = match vec.get(0) {
        Some(i) => *i,
        None => 0,
    };

    for num in vec {
        result = gcd(result, *num);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(18, 12), 6);
        assert_eq!(gcd(12, 18), 6);
        assert_eq!(gcd(1, 1), 1);
        assert_eq!(gcd(10, 0), 10);
        assert_eq!(gcd(0, 10), 10);
        assert_eq!(gcd(0, 0), 0);
    }

    #[test]
    fn test_gcd_vec() {
        assert_eq!(gcd_vec(&vec![18, 12]), 6);
        assert_eq!(gcd_vec(&vec![36, 12, 48, 120]), 12);
        assert_eq!(gcd_vec(&vec![0, 0, 0]), 0);
        assert_eq!(gcd_vec(&vec![10, 0, 25]), 5);
        assert_eq!(gcd_vec(&vec![3]), 3);
        assert_eq!(gcd_vec(&vec![]), 0);
    }
}
