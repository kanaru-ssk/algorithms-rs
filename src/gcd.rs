pub fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    }

    // 割り切れるまで再帰的に呼び出す
    gcd(b, a % b)
}

pub fn gcd_vec(vec: &Vec<u32>) -> u32 {
    let mut result = 0;

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
        let samples = [
            ([18, 12], 6),
            ([12, 18], 6),
            ([1, 1], 1),
            ([10, 0], 10),
            ([0, 10], 10),
            ([0, 0], 0),
        ];

        for &(input, expected) in &samples {
            assert_eq!(gcd(input[0], input[1]), expected);
        }
    }

    #[test]
    fn test_gcd_vec() {
        let samples = [
            (&vec![18, 12], 6),
            (&vec![36, 12, 48, 120], 12),
            (&vec![0, 0, 0], 0),
            (&vec![3], 3),
            (&vec![0], 0),
            (&vec![], 0),
        ];

        for &(input, expected) in &samples {
            assert_eq!(gcd_vec(input), expected);
        }
    }
}
