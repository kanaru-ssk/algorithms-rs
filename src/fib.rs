use super::dynamic_programming::dynamic_programming;

pub fn fib1(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n <= 2 {
        1
    } else {
        fib1(n - 1) + fib1(n - 2)
    }
}

pub fn fib2(n: usize) -> u32 {
    fn calc_cell(y: usize, x: usize, table: &Vec<Vec<u32>>, _: &Vec<u32>) -> u32 {
        if x <= 0 {
            0
        } else if x <= 2 {
            1
        } else {
            table[y][x - 1] + table[y][x - 2]
        }
    }

    dynamic_programming(&Vec::new(), n, calc_cell)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib1() {
        let samples = [
            (0, 0),
            (1, 1),
            (2, 1),
            (3, 2),
            (4, 3),
            (5, 5),
            (6, 8),
            (20, 6765),
        ];

        for &(input, expected) in &samples {
            assert_eq!(fib1(input), expected);
        }
    }

    #[test]
    fn test_fib2() {
        let samples = [
            (0, 0),
            (1, 1),
            (2, 1),
            (3, 2),
            (4, 3),
            (5, 5),
            (6, 8),
            (20, 6765),
        ];

        for &(input, expected) in &samples {
            assert_eq!(fib2(input), expected);
        }
    }
}
