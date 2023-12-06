use aoc2023::day4::card;

fn main() {
    let input = include_str!("../../data/dec4.txt");
    let part1 = part_one(input);
    // let part2 = part_two(input);
    println!("Answer for Day 4:");
    println!("\tpart 1: {part1}");
    // println!("\tpart 2: {part2}");
}

fn part_one(input: &str) -> u16 {
    input.lines().map(|l| card(l).unwrap().1.value()).sum()
}
