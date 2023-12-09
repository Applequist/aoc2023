use aoc2023::day6::{race, race_list, Race};

fn main() {
    let input = include_str!("../../data/dec6.txt");
    let part1 = part_one(input);
    let part2 = part_two(input);
    println!("Answer for Day 5:");
    println!("\tpart 1: {part1}");
    println!("\tpart 2: {part2}");
}

fn part_one(input: &str) -> u64 {
    let races = race_list(input).unwrap().1;
    races
        .iter()
        .map(|r| {
            let (l, h) = r.solve();
            h - l + 1
        })
        .product()
}

fn part_two(input: &str) -> u64 {
    let race = race(input).unwrap().1;
    println!("{:?}", race);
    let (low, high) = race.solve();
    println!("{:?}", (low, high));
    high - low + 1
}
