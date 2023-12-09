use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1},
    combinator::{eof, map_res},
    multi::separated_list1,
    sequence::{delimited, pair, preceded, terminated},
    IResult, Parser,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Race {
    duration_ms: u64,
    record_mm: u64,
}

impl Race {
    fn distance(&self, t: u64) -> u64 {
        (self.duration_ms - t) * t
    }

    pub fn solve(&self) -> (u64, u64) {
        let mut bound = (0, self.duration_ms / 2);
        loop {
            let (low, high) = bound;
            if low == high {
                break;
            }
            let mid = (high + low) / 2;
            if self.distance(mid) <= self.record_mm {
                bound = (mid + 1, high);
            } else {
                bound = (low, mid);
            }
        }
        (bound.0, self.duration_ms - bound.0)
    }
}

fn integer(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

/// A space separated list of number
fn number_list(input: &str) -> IResult<&str, Vec<u64>> {
    terminated(separated_list1(space1, integer), alt((line_ending, eof))).parse(input)
}

fn time_list(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(pair(tag("Time:"), space1), number_list).parse(input)
}

fn distance_list(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(pair(tag("Distance:"), space1), number_list).parse(input)
}

pub fn race_list(input: &str) -> IResult<&str, Vec<Race>> {
    pair(time_list, distance_list)
        .map(|(ts, ds)| {
            ts.iter()
                .zip(ds.iter())
                .map(|(&duration_ms, &record_mm)| Race {
                    duration_ms,
                    record_mm,
                })
                .collect::<Vec<_>>()
        })
        .parse(input)
}

pub fn duration(input: &str) -> IResult<&str, u64> {
    delimited(
        pair(tag("Time:"), space1),
        separated_list1(space1, digit1),
        line_ending,
    )
    .map(|g| {
        let mut s = String::new();
        g.iter().for_each(|&d| s.push_str(d));
        s.parse().unwrap()
    })
    .parse(input)
}

pub fn record(input: &str) -> IResult<&str, u64> {
    delimited(
        pair(tag("Distance:"), space1),
        separated_list1(space1, digit1),
        alt((line_ending, eof)),
    )
    .map(|g| {
        let mut s = String::new();
        g.iter().for_each(|&d| s.push_str(d));
        s.parse().unwrap()
    })
    .parse(input)
}

pub fn race(input: &str) -> IResult<&str, Race> {
    pair(duration, record)
        .map(|(d, r)| Race {
            duration_ms: d,
            record_mm: r,
        })
        .parse(input)
}

#[cfg(test)]
mod tests {
    use crate::day6::Race;

    #[test]
    fn test_distance() {
        let race = Race {
            duration_ms: 7,
            record_mm: 9,
        };
        assert_eq!(race.distance(3), 12);
        assert_eq!(race.distance(4), 12);
    }

    #[test]
    fn test_solve() {
        let race = Race {
            duration_ms: 7,
            record_mm: 9,
        };
        println!("{}", race.distance(2));
        println!("{}", race.distance(5));
        assert_eq!(race.solve(), (2, 5));

        let race2 = Race {
            duration_ms: 30,
            record_mm: 200,
        };
        println!("{}", race2.distance(10));
        println!("{}", race2.distance(20));
        assert_eq!(race2.solve(), (11, 19));
    }
}
