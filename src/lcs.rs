pub fn lcs(a: &str, b: &str) -> u32 {
    let table_h = a.len() + 1;
    let table_w = b.len() + 1;
    let mut table: Vec<Vec<u32>> = vec![vec![0; table_w]; table_h];

    for y in 1..table_h {
        for x in 1..table_w {
            if a.chars().nth(y - 1) == b.chars().nth(x - 1) {
                table[y][x] = table[y - 1][x - 1] + 1
            } else {
                table[y][x] = std::cmp::max(table[y - 1][x], table[y][x - 1])
            }
        }
    }

    table[table_h - 1][table_w - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcs() {
        let samples = [
            (
                [
                    "ACCGGTCGAGTGCGCGGAAGCCGGCCGAA",
                    "GTCGTTCGGAATGCCGTTGCTCTGTAAA",
                ],
                20,
            ),
            (["A00B0000C", "XXAXBXCXX"], 3),
        ];

        for &(input, expected) in &samples {
            assert_eq!(lcs(input[0], input[1]), expected);
        }
    }
}
