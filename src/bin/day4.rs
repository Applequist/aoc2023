use aoc2023::day4::{card, Card};

fn main() {
    let input = include_str!("../../data/dec4.txt");
    let part1 = part_one(input);
    let part2 = part_two(input);
    println!("Answer for Day 4:");
    println!("\tpart 1: {part1}");
    println!("\tpart 2: {part2}");
}

fn part_one(input: &str) -> u16 {
    input.lines().map(|l| card(l).unwrap().1.value()).sum()
}

fn part_two(input: &str) -> u32 {
    let cards: Vec<Card> = input.lines().map(|l| card(l).unwrap().1).collect();
    let mut num_cards = vec![1_u32; cards.len()];
    for i in 0..cards.len() {
        let matching = cards[i].matching;
        if matching > 0 {
            for j in i + 1..=(i + matching as usize) {
                num_cards[j] += num_cards[i];
            }
        }
    }
    num_cards.iter().sum::<u32>()
}
