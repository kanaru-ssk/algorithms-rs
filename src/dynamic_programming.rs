pub fn dynamic_programming<T, U>(
    items: &Vec<U>,
    capacity: usize,
    calc_cell: fn(y: usize, x: usize, table: &Vec<Vec<T>>, items: &Vec<U>) -> T,
) -> T {
    let table_h = items.len() + 1;
    let table_w = capacity + 1;
    let mut table: Vec<Vec<T>> = vec![];

    for y in 0..table_h {
        table.push(vec![]);
        for x in 0..table_w as usize {
            let cell = calc_cell(y, x, &table, items);
            table[y].push(cell);
        }
    }

    table.pop().unwrap().pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
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

        fn fib(y: usize, x: usize, table: &Vec<Vec<u32>>, _: &Vec<u32>) -> u32 {
            if x <= 0 {
                0
            } else if x <= 2 {
                1
            } else {
                table[y][x - 1] + table[y][x - 2]
            }
        }

        for &(input, expected) in &samples {
            assert_eq!(dynamic_programming(&Vec::new(), input, fib), expected);
        }
    }

    #[test]
    fn test_bin_packing() {
        let items: Vec<usize> = vec![4, 7, 8, 5, 1];
        let samples = [((&items, 10), true), ((&items, 22), false)];

        fn bin_packing(y: usize, x: usize, table: &Vec<Vec<bool>>, items: &Vec<usize>) -> bool {
            if y == 0 {
                x == 0
            } else if table[y - 1][x] {
                true
            } else if x < items[y - 1] {
                false
            } else {
                table[y - 1][x - items[y - 1]]
            }
        }

        for (input, expected) in samples {
            assert_eq!(dynamic_programming(input.0, input.1, bin_packing), expected);
        }
    }

    #[test]
    fn test_knapsack() {
        struct Item {
            cost: u32,
            value: u32,
        }
        let items = vec![
            Item { cost: 3, value: 4 },
            Item { cost: 4, value: 7 },
            Item { cost: 5, value: 8 },
            Item { cost: 4, value: 5 },
            Item { cost: 1, value: 1 },
        ];
        let samples = [((&items, 10), 16), ((&items, 22), 21)];

        fn knapsack(y: usize, x: usize, table: &Vec<Vec<u32>>, items: &Vec<Item>) -> u32 {
            if y == 0 {
                0
            } else if (x as u32) < items[y - 1].cost {
                table[y - 1][x]
            } else {
                std::cmp::max(
                    table[y - 1][x - items[y - 1].cost as usize] + items[y - 1].value,
                    table[y - 1][x],
                )
            }
        }

        for ((items, capacity), expected) in samples {
            assert_eq!(dynamic_programming(items, capacity, knapsack), expected);
        }
    }
}
