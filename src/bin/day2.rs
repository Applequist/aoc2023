use aoc2023::day2::{game, Sample};

fn main() {
    let input = include_str!("../../data/dec2.txt");
    let num = part_one(input);
    println!("Answer for Day 2 part 1 is {num}");
}

fn part_one(input: &str) -> u32 {
    let config = Sample::new(12, 13, 14);
    input
        .lines()
        .map(|l| game(l).unwrap().1)
        .filter(|g| g.samples.iter().all(|&s| s.is_possible(&config)))
        .map(|g| g.id)
        .sum() // 2476
}
