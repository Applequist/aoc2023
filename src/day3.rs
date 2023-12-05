//! Idea: for each line, create the following:
//! - a list of symbol position
//! - and a list of number + (start, end) positions.
//!
//! Then for each number in line N, check if there is a symbol in line N-1, N and N+1 that is
//! adjacent to it range, and if so add it to the sum of part number.
//! To avoid summing the number more than once if it is adjacent to more than one symbols
//! we first merge the symbols positions of all 3 lines (2 for first and last lines).

use nom::AsChar;

#[derive(Debug, Clone, PartialEq)]
pub struct LineIndex {
    pub symbols: Vec<usize>,
    pub numbers: Vec<((usize, usize), u32)>,
}

impl LineIndex {
    pub fn adjacent_sym(&self, range: (usize, usize)) -> bool {
        let min = if range.0 == 0 { 0 } else { range.0 - 1 };
        let max = range.1 + 1;
        let r = min..=max;
        println!("Has {:?} symbol in [{}, {}]", self, min, max);
        self.symbols.iter().any(|ix| r.contains(ix))
    }

    pub fn part_numbers<'a>(&'a self, sym: &'a [usize]) -> impl Iterator<Item = u32> + 'a {
        self.numbers.iter().filter_map(|((s, e), n)| {
            let min = if *s == 0 { 0 } else { *s - 1 };
            let max = *e + 1;
            let rng = min..=max;
            if sym.iter().any(|ix| rng.contains(ix)) {
                Some(*n)
            } else {
                None
            }
        })
    }
}

pub fn parse_line(input: &str) -> LineIndex {
    let mut symbols = vec![];
    let mut numbers = vec![];
    let mut start = None;
    for (ix, c) in input.chars().enumerate() {
        if c.is_dec_digit() && start.is_none() {
            start = Some(ix);
        } else if c.is_dec_digit() && start.is_some() {
            continue;
        } else if !c.is_dec_digit() && start.is_some() {
            let start_ix = start.unwrap();
            start = None;
            let num = input[start_ix..ix].parse::<u32>().unwrap();
            numbers.push(((start_ix, ix - 1), num));
            if c != '.' {
                symbols.push(ix);
            }
        } else if c != '.' {
            symbols.push(ix);
        }
    }
    LineIndex { symbols, numbers }
}

#[cfg(test)]
mod tests {
    use crate::day3::{parse_line, LineIndex};

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("467..114.."),
            LineIndex {
                symbols: vec![],
                numbers: vec![((0, 2), 467), ((5, 7), 114)]
            }
        );

        assert_eq!(
            parse_line("617*......"),
            LineIndex {
                symbols: vec![3],
                numbers: vec![((0, 2), 617)],
            }
        )
    }

    #[test]
    fn test_adjacent_sym() {
        let line_index = parse_line("617*......");

        assert!(line_index.adjacent_sym((0, 2)));
        assert!(line_index.adjacent_sym((3, 3)));
        assert!(line_index.adjacent_sym((4, 6)));
        assert!(!line_index.adjacent_sym((5, 7)));
    }

    #[test]
    fn test_part_numbers() {
        let line_index = parse_line("617*...54..");
        let next_index = parse_line("....../....");
        let mut all_sym = line_index.symbols.clone();
        all_sym.extend(next_index.symbols);
        assert_eq!(
            line_index.part_numbers(&all_sym).collect::<Vec<_>>(),
            &[617, 54]
        );
    }
}
