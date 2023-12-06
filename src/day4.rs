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
    pub matching: u16,
}

impl Card {
    pub fn new(id: u16, winnings: Vec<u16>, numbers: Vec<u16>) -> Self {
        let matching = numbers.iter().filter(|n| winnings.contains(n)).count() as u16;
        Self {
            id,
            winnings,
            numbers,
            matching,
        }
    }

    pub fn value(&self) -> u16 {
        if self.matching == 0 {
            0
        } else {
            2_u16.pow((self.matching - 1) as u32)
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
    .map(|(id, _, _, winnings, _, _, numbers)| Card::new(id, winnings, numbers))
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
            Card::new(
                1,
                vec![41, 48, 83, 86, 17],
                vec![83, 86, 6, 31, 17, 9, 48, 53]
            )
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
