use std::collections::HashMap;

pub fn prime_factorize(mut n: u32) -> HashMap<u32, u32> {
    let mut result: HashMap<u32, u32> = HashMap::new();

    if n == 0 {
        return result;
    }

    let mut i = 1;
    while i * i <= n {
        i += 1;
        if n % i != 0 {
            continue;
        }

        let mut e = 0;
        while n % i == 0 {
            e += 1;
            n /= i;
        }

        result.insert(i, e);
    }

    if n != 1 {
        result.insert(n, 1);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_factorize() {
        let samples = [
            (0, HashMap::from([])),
            (1, HashMap::from([])),
            (2, HashMap::from([(2, 1)])),
            (12, HashMap::from([(2, 2), (3, 1)])),
            (53, HashMap::from([(53, 1)])),
            (4876, HashMap::from([(2, 2), (23, 1), (53, 1)])),
        ];

        for (input, expected) in samples {
            assert_eq!(prime_factorize(input), expected);
        }
    }
}
