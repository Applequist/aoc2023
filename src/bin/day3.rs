use aoc2023::day3::{numbers, SymbolMap};

fn main() {
    let input = include_str!("../../data/dec3.txt");
    let symbols = SymbolMap::parse(input);
    let num = numbers(input);

    let lines: Vec<_> = input.lines().collect();
    let (line_count, col_count) = (lines.len(), lines.iter().map(|s| s.len()).max().unwrap());
    let sum: u64 = num
        .iter()
        .filter_map(|(r, (s, e), n)| {
            let min_row = if *r == 0 { 0 } else { *r - 1 };
            let max_row = if *r == line_count - 1 { *r } else { *r + 1 };
            let min_col = if *s == 0 { 0 } else { *s - 1 };
            let max_col = if *e == col_count - 1 { *e } else { *e + 1 };
            let symbols = symbols.query((min_row, max_row), (min_col, max_col));
            if !symbols.is_empty() {
                // println!("PART NUMBER: {n}");
                Some(n)
            } else {
                println!("Not a part number: line {} number {}", r + 1, n);
                (min_row..=max_row).for_each(|r| {
                    println!("{}", &lines[r][min_col..=max_col]);
                });
                None
            }
        })
        .sum();
    println!("sum = {sum}");
}
