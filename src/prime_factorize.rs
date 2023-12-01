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
    fn return_correct_value() {
        let result = prime_factorize(0);
        assert_eq!(result.len(), 0);

        let result = prime_factorize(1);
        assert_eq!(result.len(), 0);

        let result = prime_factorize(2);
        assert_eq!(result.len(), 1);
        assert_eq!(result.get(&2), Some(&1));

        let result = prime_factorize(12);
        assert_eq!(result.len(), 2);
        assert_eq!(result.get(&2), Some(&2));
        assert_eq!(result.get(&3), Some(&1));

        let result = prime_factorize(53);
        assert_eq!(result.len(), 1);
        assert_eq!(result.get(&53), Some(&1));

        let result = prime_factorize(4876);
        assert_eq!(result.len(), 3);
        assert_eq!(result.get(&2), Some(&2));
        assert_eq!(result.get(&23), Some(&1));
        assert_eq!(result.get(&53), Some(&1));
    }
}
