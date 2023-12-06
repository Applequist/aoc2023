//! Idea: for each line, create the following:
//! - a list of symbol position
//! - and a list of number + (start, end) positions.
//!
//! Then for each number in line N, check if there is a symbol in line N-1, N and N+1 that is
//! adjacent to it range, and if so add it to the sum of part number.
//! To avoid summing the number more than once if it is adjacent to more than one symbols
//! we first merge the symbols positions of all 3 lines (2 for first and last lines).

/// A sparse symbol map: for each symbol, it stores the line and column indices (0 to N) in the
/// input.
#[derive(Debug, Clone, PartialEq)]
pub struct SymbolMap {
    symbols: Vec<(usize, usize)>,
}

impl SymbolMap {
    pub fn parse(input: &str) -> Self {
        let symbols = input
            .lines()
            .enumerate()
            .flat_map(|(r, l)| {
                println!("Extracting symbols from line {r}: '{l}'");
                l.chars()
                    .enumerate()
                    .filter_map(|(c, a)| {
                        if a != '.' && !a.is_ascii_digit() {
                            println!("\tsym '{a}' at ({r}, {c})");
                            Some((r, c))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        SymbolMap { symbols }
    }

    pub fn query(&self, rows: (usize, usize), columns: (usize, usize)) -> Vec<(usize, usize)> {
        self.symbols
            .iter()
            .filter(|(r, c)| *r >= rows.0 && *r <= rows.1 && *c >= columns.0 && *c <= columns.1)
            .cloned()
            .collect()
    }
}

pub fn numbers(input: &str) -> Vec<(usize, (usize, usize), u64)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(r, l)| {
            println!("Extracting numbers from line {r}: '{l}'");
            let mut numbers = vec![];
            let mut start = None;
            for (c, a) in l.chars().enumerate() {
                if start.is_none() {
                    if a.is_ascii_digit() {
                        start = Some(c);
                    }
                } else if !a.is_ascii_digit() {
                    let start_ix = start.unwrap();
                    start = None;
                    let num = l[start_ix..c].parse::<u64>().unwrap();
                    numbers.push((r, (start_ix, c - 1), num));
                    let s = if start_ix == 0 { 0 } else { start_ix - 1 };
                    println!("\tnum {} ('{}')", num, &l[s..=c]);
                } else {
                    continue;
                }
            }
            if let Some(s) = start {
                let num = l[s..].parse::<u64>().unwrap();
                numbers.push((r, (s, l.len() - 1), num));
                let ctx_s = if s == 0 { 0 } else { s - 1 };
                println!("\tnum {} ('{}')", num, &l[ctx_s..]);
            }
            numbers
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::day3::{numbers, SymbolMap};

    #[test]
    fn test_symbolmap() {
        let input = include_str!("../data/dec3_sample.txt");
        assert_eq!(
            SymbolMap::parse(input),
            SymbolMap {
                symbols: vec![(1, 3), (3, 6), (4, 3), (5, 5), (8, 3), (8, 5)]
            }
        );
    }

    #[test]
    fn test_numbers() {
        let input = include_str!("../data/dec3_sample.txt");
        assert_eq!(
            numbers(input),
            vec![
                (0, (0, 2), 467),
                (0, (5, 7), 114),
                (2, (2, 3), 35),
                (2, (6, 8), 633),
                (4, (0, 2), 617),
                (5, (7, 8), 58),
                (6, (2, 4), 592),
                (7, (6, 8), 755),
                (9, (1, 3), 664),
                (9, (5, 7), 598),
            ]
        );
    }
}
