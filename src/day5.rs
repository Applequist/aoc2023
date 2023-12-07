use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{char, digit1, line_ending, space1},
    combinator::{eof, map_res},
    multi::{count, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
    IResult, Parser,
};

/// An interval (src, len) = src..src+len
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interval(u64, u64);

impl Interval {
    pub fn contains(&self, v: u64) -> bool {
        self.0 <= v && v < self.0 + self.1
    }
}

/// An integer mapping (Interval, dst) maps integer in Interval to an interval of the same lenght
/// starting at dst.
/// Eg ((2, 2), 1) maps 2 and 3 to 1 and 2 respectively.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mapping {
    source: Interval,
    dst: u64,
}

impl Mapping {
    pub fn map(&self, v: u64) -> u64 {
        assert!(self.source.contains(v));
        self.dst + (v - self.source.0)
    }
}

impl From<(u64, u64, u64)> for Mapping {
    /// Create a Mapping from (dst, src, len).
    fn from(value: (u64, u64, u64)) -> Self {
        Self {
            source: Interval(value.1, value.2),
            dst: value.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntervalMap {
    mappings: Vec<Mapping>,
}

impl IntervalMap {
    pub fn map(&self, n: u64) -> u64 {
        let rng_mapping = self.mappings.iter().find(|&m| m.source.contains(n));
        if let Some(m) = rng_mapping {
            m.map(n)
        } else {
            n
        }
    }
}

fn integer(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

/// A space separated list of number
fn number_list(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, integer).parse(input)
}

fn seeds(input: &str) -> IResult<&str, (Vec<u64>, Vec<Interval>)> {
    tuple((tag("seeds:"), space1, number_list, line_ending))
        .map(|(_, _, n, _)| {
            (
                n.clone(),
                n.chunks_exact(2).map(|s| Interval(s[0], s[1])).collect(),
            )
        })
        .parse(input)
}

fn header(input: &str) -> IResult<&str, &str> {
    tuple((is_not(":"), char(':'), line_ending))
        .map(|(h, _, _)| h)
        .parse(input)
}

fn mapping(input: &str) -> IResult<&str, Mapping> {
    number_list
        .map(|n| Mapping::from((n[0], n[1], n[2])))
        .parse(input)
}

fn interval_map(input: &str) -> IResult<&str, IntervalMap> {
    tuple((header, separated_list1(line_ending, mapping)))
        .map(|(_, m)| IntervalMap { mappings: m })
        .parse(input)
}

pub fn seeds_and_maps(input: &str) -> IResult<&str, ((Vec<u64>, Vec<Interval>), Vec<IntervalMap>)> {
    tuple((
        seeds,
        line_ending,
        separated_list1(line_ending, interval_map),
    ))
    .map(|(s, _, m)| (s, m))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use crate::day5::{interval_map, mapping, seeds, Interval, IntervalMap, Mapping};

    #[test]
    fn test_seeds() {
        assert_eq!(
            seeds("seeds: 1 2 3 4\n"),
            Ok(("", (vec![1, 2, 3, 4], vec![Interval(1, 2), Interval(3, 4)])))
        );
    }

    #[test]
    fn test_mapping() {
        assert_eq!(
            mapping("50 98 2"),
            Ok((
                "",
                Mapping {
                    source: Interval(98, 2),
                    dst: 50
                }
            ))
        );
    }

    #[test]
    fn test_interval_map() {
        assert_eq!(
            interval_map("seed-to-soil map:\n1 2 3\n4 5 6"),
            Ok((
                "",
                IntervalMap {
                    mappings: vec![
                        Mapping {
                            source: Interval(2, 3),
                            dst: 1
                        },
                        Mapping {
                            source: Interval(5, 6),
                            dst: 4
                        },
                    ]
                }
            ))
        );
    }
    #[test]
    fn test_interval_contains() {
        let a = Interval(10, 2);
        assert!(!a.contains(9));
        assert!(a.contains(10));
        assert!(a.contains(11));
        assert!(!a.contains(12));
    }

    #[test]
    fn test_mapping_map() {
        let mapping = Mapping::from((50, 98, 2));
        assert_eq!(mapping.map(98), 50);
        assert_eq!(mapping.map(99), 51);
    }

    #[test]
    fn test_intervalmap_map() {
        let rng = IntervalMap {
            mappings: vec![Mapping::from((50, 98, 2)), Mapping::from((52, 50, 48))],
        };
        assert_eq!(rng.map(10), 10);
        assert_eq!(rng.map(50), 52);
        assert_eq!(rng.map(98), 50);
    }
}
