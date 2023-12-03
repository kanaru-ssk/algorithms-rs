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
        let samples = [
            ([18, 12], 36),
            ([12, 18], 36),
            ([1, 1], 1),
            ([10, 0], 0),
            ([0, 10], 0),
            ([0, 0], 0),
        ];

        for &(input, expected) in &samples {
            assert_eq!(lcm(input[0], input[1]), expected);
        }
    }

    #[test]
    fn test_lcm_vec() {
        let samples = [
            (&vec![18, 12], 36),
            (&vec![36, 12, 48, 120], 720),
            (&vec![0, 0, 0], 0),
            (&vec![3], 3),
            (&vec![0], 0),
            (&vec![], 0),
        ];

        for &(input, expected) in &samples {
            assert_eq!(lcm_vec(input), expected);
        }
    }
}
