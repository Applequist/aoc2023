use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::tuple,
    IResult, Parser,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    id: u16,
    winnings: Vec<u16>,
    numbers: Vec<u16>,
}

impl Card {
    pub fn value(&self) -> u16 {
        let pow = self
            .numbers
            .iter()
            .filter(|n| self.winnings.contains(n))
            .count();
        if pow == 0 {
            0
        } else {
            2_u16.pow((pow - 1) as u32)
        }
    }
}

pub fn card(input: &str) -> IResult<&str, Card> {
    tuple((
        card_id,
        tag(":"),
        space1,
        number_list,
        tag(" |"),
        space1,
        number_list,
    ))
    .map(|(id, _, _, winnings, _, _, numbers)| Card {
        id,
        winnings,
        numbers,
    })
    .parse(input)
}

fn card_id(input: &str) -> IResult<&str, u16> {
    tuple((tag("Card"), space1, map_res(digit1, str::parse)))
        .map(|t| t.2)
        .parse(input)
}

fn number_list(input: &str) -> IResult<&str, Vec<u16>> {
    separated_list1(space1, map_res(digit1, str::parse)).parse(input)
}

#[cfg(test)]
mod tests {
    use crate::day4::{card, Card};

    #[test]
    fn test_card() {
        assert_eq!(
            card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")
                .unwrap()
                .1,
            Card {
                id: 1,
                winnings: vec![41, 48, 83, 86, 17],
                numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
            }
        );
    }

    #[test]
    fn test_value() {
        assert_eq!(
            card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")
                .unwrap()
                .1
                .value(),
            8
        );
    }
}
