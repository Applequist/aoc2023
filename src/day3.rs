//! Part 1: for each line, create the following:
//! - a list of symbol position
//! - and a list of number + line and (start, end) positions.
//!
//! Then for each number query the symbol in the number bounding box. If not empty sum it.
//!
//! Part 2: Build the same symbol and numbers. But this time query the number in the surround gear
//! symbol, if exactly 2, multiply them and sum the result.

/// A sparse symbol map: for each symbol, it stores the line and column indices (0 to N) in the
/// input.
#[derive(Debug, Clone, PartialEq)]
pub struct SymbolMap {
    size: (usize, usize),
    symbols: Vec<((usize, usize), char)>,
}

impl SymbolMap {
    pub fn new(lines: &[&str]) -> Self {
        let size = (lines.len(), lines.iter().map(|s| s.len()).max().unwrap());

        let symbols = lines
            .iter()
            .enumerate()
            .flat_map(|(r, l)| {
                l.chars()
                    .enumerate()
                    .filter_map(|(c, a)| {
                        if a != '.' && !a.is_ascii_digit() {
                            Some(((r, c), a))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        SymbolMap { size, symbols }
    }

    pub fn query(
        &self,
        rows: (usize, usize),
        columns: (usize, usize),
    ) -> Vec<((usize, usize), char)> {
        self.symbols
            .iter()
            .filter(|((r, c), _)| {
                *r >= rows.0 && *r <= rows.1 && *c >= columns.0 && *c <= columns.1
            })
            .cloned()
            .collect()
    }

    pub fn gears<'a>(&'a self) -> impl Iterator<Item = ((usize, usize), char)> + 'a {
        self.symbols.iter().filter(|((_, _), a)| *a == '*').cloned()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NumberMap {
    pub size: (usize, usize),
    pub numbers: Vec<(usize, (usize, usize), u64)>,
}

impl NumberMap {
    pub fn new(lines: &[&str]) -> Self {
        let size = (lines.len(), lines.iter().map(|s| s.len()).max().unwrap());
        let numbers = lines
            .iter()
            .enumerate()
            .flat_map(|(r, l)| {
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
                    } else {
                        continue;
                    }
                }
                if let Some(s) = start {
                    let num = l[s..].parse::<u64>().unwrap();
                    numbers.push((r, (s, l.len() - 1), num));
                }
                numbers
            })
            .collect();
        Self { size, numbers }
    }

    pub fn query(&self, pos: (usize, usize)) -> Vec<u64> {
        self.numbers
            .iter()
            .filter_map(|(l, (s, e), n)| {
                if *l < pos.0 - 1 || *l > pos.0 + 1 || *e < pos.1 - 1 || *s > pos.1 + 1 {
                    None
                } else {
                    Some(*n)
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::day3::{NumberMap, SymbolMap};

    #[test]
    fn test_symbolmap() {
        let input = include_str!("../data/dec3_sample.txt");
        let lines = input.lines().collect::<Vec<_>>();
        assert_eq!(
            SymbolMap::new(&lines),
            SymbolMap {
                size: (10, 10),
                symbols: vec![
                    ((1, 3), '*'),
                    ((3, 6), '#'),
                    ((4, 3), '*'),
                    ((5, 5), '+'),
                    ((8, 3), '$'),
                    ((8, 5), '*')
                ]
            }
        );
    }

    #[test]
    fn test_numbermap() {
        let input = include_str!("../data/dec3_sample.txt");
        let lines = input.lines().collect::<Vec<_>>();
        assert_eq!(
            NumberMap::new(&lines).numbers,
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
