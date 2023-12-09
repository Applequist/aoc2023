use nom::{
    bytes::complete::{is_not, tag},
    character::complete::{char, digit1, line_ending, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::tuple,
    IResult, Parser,
};
use std::{fmt::Display, iter::Iterator};

/// An **non-empty** closed interval (src, len) containing interger >= src and < src + len.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interval {
    pub min: u64,
    pub max: u64,
}

impl Interval {
    pub fn min_max(min: u64, max: u64) -> Self {
        assert!(max >= min);
        Self { min, max }
    }

    pub fn start_len(start: u64, len: u64) -> Self {
        assert!(len > 0);
        Self {
            min: start,
            max: start + len - 1,
        }
    }

    #[inline]
    pub fn len(&self) -> u64 {
        self.max + 1 - self.min
    }

    #[inline]
    pub fn lt(&self, v: u64) -> bool {
        self.max < v
    }

    #[inline]
    pub fn gt(&self, v: u64) -> bool {
        v < self.min
    }

    /// Return `true` if this interval contains v.
    #[inline]
    pub fn contains(&self, v: u64) -> bool {
        self.min <= v && v <= self.max
    }

    #[inline]
    pub fn overlap(&self, other: Interval) -> bool {
        self.max >= other.min && self.min <= other.max
    }

    #[inline]
    pub fn touch(&self, other: Interval) -> bool {
        self.max + 1 == other.min || other.max + 1 == self.min
    }

    #[inline]
    pub fn include(&self, other: Interval) -> bool {
        self.min <= other.min && other.max <= self.max
    }

    /// Split this interval into a *left* interval of values less than v and
    /// a *right* interval of values greater than or equal to v.
    pub fn split_before(&self, v: u64) -> (Option<Interval>, Option<Interval>) {
        if v <= self.min {
            (None, Some(*self))
        } else if v > self.max {
            (Some(*self), None)
        } else {
            (
                Some(Interval::min_max(self.min, v - 1)),
                Some(Interval::min_max(v, self.max)),
            )
        }
    }

    /// Split this interval into a *left* interval of values less than or equal to v and
    /// a *right* interval of values greater than v.
    pub fn split_after(&self, v: u64) -> (Option<Interval>, Option<Interval>) {
        if v >= self.max {
            (Some(*self), None)
        } else if v < self.min {
            (None, Some(*self))
        } else {
            (
                Some(Interval::min_max(self.min, v)),
                Some(Interval::min_max(v + 1, self.max)),
            )
        }
    }
}

impl Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}..{}]", self.min, self.max)
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
    pub fn source(&self) -> Interval {
        self.source
    }

    pub fn destination(&self) -> Interval {
        Interval::start_len(self.dst, self.source.len())
    }

    pub fn map(&self, v: u64) -> u64 {
        assert!(self.source.contains(v));
        self.dst + (v - self.source.min)
    }
}

impl Display for Mapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.source, self.destination())
    }
}

impl From<(u64, u64, u64)> for Mapping {
    /// Create a Mapping from (dst, src, len).
    fn from(value: (u64, u64, u64)) -> Self {
        Self {
            source: Interval::start_len(value.1, value.2),
            dst: value.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntervalMap {
    mappings: Vec<Mapping>,
}

impl IntervalMap {
    pub fn new(mut mappings: Vec<Mapping>) -> Self {
        mappings.as_mut_slice().sort_by(|a, b| {
            let a_min = a.source.min;
            let b_min = b.source.min;
            a_min.partial_cmp(&b_min).unwrap()
        });
        assert!(mappings
            .as_slice()
            .windows(2)
            .all(|w| w[0].source.lt(w[1].source.min)));
        Self { mappings }
    }

    pub fn map(&self, n: u64) -> u64 {
        let rng_mapping = self.mappings.iter().find(|&m| m.source.contains(n));
        if let Some(m) = rng_mapping {
            m.map(n)
        } else {
            n
        }
    }

    pub fn map_interval(&self, interval: Interval) -> Vec<Interval> {
        print!("{}", interval);
        let mut unmapped = Some(interval);
        let mut mapped = vec![];
        for m in self.mappings.iter() {
            if m.source.lt(unmapped.unwrap().min) {
                continue;
            }
            if m.source.gt(unmapped.unwrap().max) {
                mapped.push(unmapped.unwrap());
                unmapped = None;
                break;
            }
            let (l, r) = unmapped.unwrap().split_before(m.source.min);
            if let Some(i) = l {
                mapped.push(i);
            }
            unmapped = r;
            if unmapped.is_none() {
                break;
            }

            let (l, r) = unmapped.unwrap().split_after(m.source.max);
            if let Some(i) = l {
                mapped.push(Interval::start_len(m.map(i.min), i.len()));
            }
            unmapped = r;
            if unmapped.is_none() {
                break;
            }
        }

        // if all mapping are less than the interval push as is.
        if let Some(i) = unmapped {
            mapped.push(i);
        }

        println!(" -> {:?}", mapped);
        mapped
    }
}

impl Display for IntervalMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for m in &self.mappings {
            write!(f, "{m}\n")?;
        }
        Ok(())
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
                n.chunks_exact(2)
                    .map(|s| Interval::start_len(s[0], s[1]))
                    .collect(),
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
        .map(|(_, m)| IntervalMap::new(m))
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
            Ok((
                "",
                (
                    vec![1, 2, 3, 4],
                    vec![Interval::start_len(1, 2), Interval::start_len(3, 4)]
                )
            ))
        );
    }

    #[test]
    fn test_mapping() {
        assert_eq!(
            mapping("50 98 2"),
            Ok((
                "",
                Mapping {
                    source: Interval::start_len(98, 2),
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
                IntervalMap::new(vec![
                    Mapping {
                        source: Interval::start_len(2, 3),
                        dst: 1
                    },
                    Mapping {
                        source: Interval::start_len(5, 6),
                        dst: 4
                    },
                ])
            ))
        );
    }

    #[test]
    fn test_interval_contains() {
        let a = Interval::start_len(10, 2);
        assert!(!a.contains(9));
        assert!(a.contains(10));
        assert!(a.contains(11));
        assert!(!a.contains(12));
    }

    #[test]
    fn test_interval_overlap() {
        let a = Interval::start_len(1, 3);
        let b = Interval::start_len(3, 5);
        let c = Interval::start_len(4, 2);
        let d = Interval::start_len(6, 3);

        assert!(a.overlap(b));
        assert!(!a.overlap(c));
        assert!(b.overlap(c));
        assert!(!c.overlap(d));
    }

    #[test]
    fn test_interval_touch() {
        let a = Interval::min_max(1, 2); // [1, 2]
        let b = Interval::min_max(3, 3); // [3]
        let c = Interval::min_max(4, 5); // [4, 5]
        assert!(a.touch(b));
        assert!(b.touch(c));
    }

    #[test]
    fn test_interval_split_before() {
        let a = Interval::min_max(1, 10);
        assert_eq!(a.split_before(1), (None, Some(a)));
        assert_eq!(
            a.split_before(5),
            (
                Some(Interval::min_max(1, 4)),
                Some(Interval::min_max(5, 10))
            )
        );
        assert_eq!(
            a.split_before(10),
            (
                Some(Interval::min_max(1, 9)),
                Some(Interval::min_max(10, 10))
            )
        );
        assert_eq!(a.split_before(11), (Some(a), None));
    }

    #[test]
    fn test_interval_split_after() {
        let a = Interval::min_max(1, 10);
        assert_eq!(a.split_after(0), (None, Some(a)));
        assert_eq!(
            a.split_after(1),
            (
                Some(Interval::min_max(1, 1)),
                Some(Interval::min_max(2, 10))
            )
        );
        assert_eq!(
            a.split_after(5),
            (
                Some(Interval::min_max(1, 5)),
                Some(Interval::min_max(6, 10))
            )
        );
        assert_eq!(a.split_after(10), (Some(a), None));
    }

    #[test]
    fn test_mapping_map() {
        let mapping = Mapping::from((50, 98, 2));
        assert_eq!(mapping.map(98), 50);
        assert_eq!(mapping.map(99), 51);
    }

    #[test]
    fn test_intervalmap_map() {
        let rng = IntervalMap::new(vec![
            Mapping::from((50, 98, 2)),
            Mapping::from((52, 50, 48)),
        ]);
        assert_eq!(rng.map(10), 10);
        assert_eq!(rng.map(50), 52);
        assert_eq!(rng.map(98), 50);
    }

    #[test]
    fn test_intervalmap_map_interval() {
        let map = IntervalMap::new(vec![
            Mapping::from((50, 98, 2)),  // [98, 99] -> [50, 51]
            Mapping::from((52, 50, 48)), // [50..97] -> [52..100]
        ]);
        let a = Interval::min_max(10, 15);
        assert_eq!(map.map_interval(a), vec![a]);

        let b = Interval::min_max(60, 97);
        assert_eq!(map.map_interval(b), vec![Interval::min_max(62, 99)]);

        let c = Interval::min_max(95, 99);
        assert_eq!(
            map.map_interval(c),
            vec![Interval::min_max(97, 99), Interval::min_max(50, 51),]
        )
    }
}
