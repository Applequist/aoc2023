use aoc2023::day3::{NumberMap, SymbolMap};

fn main() {
    let input = include_str!("../../data/dec3.txt");
    let lines: Vec<_> = input.lines().collect();
    let maps = (SymbolMap::new(&lines), NumberMap::new(&lines));
    let part1 = part_one(&maps);
    let part2 = part_two(&maps);
    println!("Answer for Day 3:");
    println!("\tpart 1: {part1}");
    println!("\tpart 2: {part2}");
}

fn part_one((ref symbols, ref numbers): &(SymbolMap, NumberMap)) -> u64 {
    numbers
        .numbers
        .iter()
        .filter_map(|(r, (s, e), n)| {
            let (line_count, col_count) = numbers.size;
            let min_row = if *r == 0 { 0 } else { *r - 1 };
            let max_row = if *r == line_count - 1 { *r } else { *r + 1 };
            let min_col = if *s == 0 { 0 } else { *s - 1 };
            let max_col = if *e == col_count - 1 { *e } else { *e + 1 };
            let symbols = symbols.query((min_row, max_row), (min_col, max_col));
            if !symbols.is_empty() {
                Some(n)
            } else {
                None
            }
        })
        .sum()
}

fn part_two((ref symbols, ref numbers): &(SymbolMap, NumberMap)) -> u64 {
    symbols
        .gears()
        .filter_map(|((r, c), _)| {
            let numbers = numbers.query((r, c));
            if numbers.len() == 2 {
                Some(numbers.iter().product::<u64>())
            } else {
                None
            }
        })
        .sum()
}
