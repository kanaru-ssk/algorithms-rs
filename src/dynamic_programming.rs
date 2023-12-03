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
        fn calc_cell(y: usize, x: usize, table: &Vec<Vec<u32>>, _: &Vec<u32>) -> u32 {
            if x <= 0 {
                0
            } else if x <= 2 {
                1
            } else {
                table[y][x - 1] + table[y][x - 2]
            }
        }
        assert_eq!(dynamic_programming(&Vec::new(), 6, calc_cell), 8);
    }

    #[test]
    fn test_bin_packing() {
        let capacity = 10;
        let items: Vec<usize> = vec![4, 7, 8, 5, 1];
        fn calc_cell(y: usize, x: usize, table: &Vec<Vec<bool>>, items: &Vec<usize>) -> bool {
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
        assert_eq!(dynamic_programming(&items, capacity, calc_cell), true);
    }

    #[test]
    fn test_knapsack() {
        struct Item {
            cost: u32,
            value: u32,
        }
        let capacity = 10;
        let items = vec![
            Item { cost: 3, value: 4 },
            Item { cost: 4, value: 7 },
            Item { cost: 5, value: 8 },
            Item { cost: 4, value: 5 },
            Item { cost: 1, value: 1 },
        ];

        fn calc_cell(y: usize, x: usize, table: &Vec<Vec<u32>>, items: &Vec<Item>) -> u32 {
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
        assert_eq!(dynamic_programming(&items, capacity, calc_cell), 16);
    }
}
