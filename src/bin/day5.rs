use aoc2023::day5::{seeds_and_maps, IntervalMap};

fn main() {
    let input = include_str!("../../data/dec5_sample.txt");
    let ((seeds, seed_ranges), maps) = seeds_and_maps(input).unwrap().1;

    let part1 = part_one((&seeds, &maps));
    // let part2 = part_two((&seed_ranges, &maps));
    println!("Answer for Day 5:");
    println!("\tpart 1: {part1}");
    // println!("\tpart 2: {part2}");
}

fn part_one((seeds, maps): (&[u64], &[IntervalMap])) -> u64 {
    seeds
        .iter()
        .map(|s| {
            let mut v = *s;
            for m in maps {
                v = m.map(v);
            }
            v
        })
        .min()
        .unwrap()
}

// fn part_two((seed_ranges, maps): (&[Rng], &[RangeMap])) -> u64 {
//     0
// }
