use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{pair, separated_pair, tuple},
    IResult, Parser,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    pub id: u32,
    pub samples: Vec<Sample>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sample {
    red: u32,
    green: u32,
    blue: u32,
}

impl Sample {
    pub fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }

    pub fn is_possible(&self, config: &Sample) -> bool {
        self.red <= config.red && self.green <= config.green && self.blue <= config.blue
    }
}

fn from_counts(counts: Vec<(u32, &str)>) -> Sample {
    let red = get_count("red", &counts);
    let green = get_count("green", &counts);
    let blue = get_count("blue", &counts);
    Sample::new(red, green, blue)
}

fn get_count(color: &str, counts: &[(u32, &str)]) -> u32 {
    counts
        .iter()
        .find(|i| i.1 == color)
        .map(|i| i.0)
        .unwrap_or_default()
}

fn color_count(input: &str) -> IResult<&str, (u32, &str)> {
    tuple((
        map_res(digit1, str::parse),
        space1,
        alt((tag("red"), tag("green"), tag("blue"))),
    ))
    .map(|t| (t.0, t.2))
    .parse(input)
}

fn sample(input: &str) -> IResult<&str, Sample> {
    separated_list1(tag(", "), color_count)
        .map(from_counts)
        .parse(input)
}

fn sample_list(input: &str) -> IResult<&str, Vec<Sample>> {
    separated_list1(tag("; "), sample).parse(input)
}

fn game_id(input: &str) -> IResult<&str, u32> {
    pair(tag("Game "), map_res(digit1, str::parse))
        .map(|t| t.1)
        .parse(input)
}

pub fn game(input: &str) -> IResult<&str, Game> {
    separated_pair(game_id, tag(": "), sample_list)
        .map(|(id, s)| Game { id, samples: s })
        .parse(input)
}

#[cfg(test)]
mod tests {

    use crate::day2::{color_count, game, game_id, sample, sample_list, Game, Sample};

    #[test]
    fn test_color_count() {
        assert_eq!(color_count("4 red"), Ok(("", (4, "red"))));
    }

    #[test]
    fn test_sample() {
        assert_eq!(
            sample("4 red, 3 green, 1 blue"),
            Ok((
                "",
                Sample {
                    red: 4,
                    green: 3,
                    blue: 1
                }
            ))
        );
        assert_eq!(
            sample("4 blue, 3 red"),
            Ok((
                "",
                Sample {
                    red: 3,
                    green: 0,
                    blue: 4
                }
            ))
        );
    }

    #[test]
    fn test_sample_list() {
        let samples = "11 blue, 13 red, 3 green; 13 red, 1 green, 6 blue";
        assert_eq!(
            sample_list(samples),
            Ok(("", vec![Sample::new(13, 3, 11), Sample::new(13, 1, 6)]))
        );
    }

    #[test]
    fn test_game_id() {
        assert_eq!(game_id("Game 12"), Ok(("", 12)));
    }

    #[test]
    fn test_game() {
        let game_spec = "Game 9: 11 blue, 13 red, 3 green; 13 red, 1 green, 6 blue";
        assert_eq!(
            game(game_spec),
            Ok((
                "",
                Game {
                    id: 9,
                    samples: vec![Sample::new(13, 3, 11), Sample::new(13, 1, 6)]
                }
            ))
        );
    }
}
