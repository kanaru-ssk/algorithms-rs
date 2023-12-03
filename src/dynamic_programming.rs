use super::gcd::gcd_vec;
use velcro::vec;

pub struct Item {
    id: u32,
    cost: u32,
    value: u32,
}

pub fn dynamic_programming(capacity: u32, items: &Vec<Item>) -> u32 {
    // 最大公約数で割ってtableの大きさを最適化
    let costs: Vec<u32> = items.iter().map(|item| item.cost).collect();
    let gcd: u32 = gcd_vec(&vec![capacity, ..costs]);
    let capacity = capacity / gcd;
    let items: Vec<Item> = items
        .iter()
        .map(|item| Item {
            id: item.id,
            cost: item.cost / gcd,
            value: item.value,
        })
        .collect();

    let num_item = items.len();
    let mut table: Vec<Vec<u32>> = vec![vec![0; capacity as usize + 1]; num_item];

    for i in 1..num_item {
        for j in 0..=capacity as usize {
            if items[i].cost > j as u32 {
                // items[i]が入らない場合 : 上のマスを代入
                table[i][j] = table[i - 1][j];
            } else {
                // items[i]が入る場合 : 上のマスと比較して大きい方を代入
                table[i][j] = std::cmp::max(
                    table[i - 1][j - items[i].cost as usize] + items[i].value,
                    table[i - 1][j],
                )
            }
        }
    }

    table[num_item - 1][capacity as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bin_packing() {
        let items = self::vec![
            Item {
                id: 1,
                cost: 150,
                value: 150
            },
            Item {
                id: 2,
                cost: 200,
                value: 200
            },
            Item {
                id: 3,
                cost: 250,
                value: 250
            },
            Item {
                id: 4,
                cost: 200,
                value: 200
            },
            Item {
                id: 5,
                cost: 50,
                value: 50
            }
        ];

        let capacity = 500;
        let result = dynamic_programming(capacity, &items);
        assert_eq!(result, 500);
        assert_eq!(result == capacity, true);

        let capacity = 720;
        let result = dynamic_programming(capacity, &items);
        assert_eq!(result, 700);
        assert_eq!(result == capacity, false);
    }

    #[test]
    fn test_knapsack() {
        let items = self::vec![
            Item {
                id: 1,
                cost: 150,
                value: 4
            },
            Item {
                id: 2,
                cost: 200,
                value: 7
            },
            Item {
                id: 3,
                cost: 250,
                value: 8
            },
            Item {
                id: 4,
                cost: 200,
                value: 5
            },
            Item {
                id: 5,
                cost: 50,
                value: 1
            }
        ];

        let capacity = 500;
        let result = dynamic_programming(capacity, &items);
        assert_eq!(result, 16);

        let capacity = 720;
        let result = dynamic_programming(capacity, &items);
        assert_eq!(result, 21);
    }
}
