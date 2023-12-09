use aoc2023::day5::{seeds_and_maps, Interval, IntervalMap};

fn main() {
    let input = include_str!("../../data/dec5.txt");
    let ((seeds, seed_ranges), maps) = seeds_and_maps(input).unwrap().1;

    let part1 = part_one((&seeds, &maps));
    let part2 = part_two((&seed_ranges, &maps));
    println!("Answer for Day 5:");
    println!("\tpart 1: {part1}");
    println!("\tpart 2: {part2}");
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

fn part_two((seed_ranges, maps): (&[Interval], &[IntervalMap])) -> u64 {
    let mut unmapped = Vec::new();
    unmapped.extend_from_slice(seed_ranges);
    unmapped.sort_by(|a, b| a.min.partial_cmp(&b.min).unwrap());
    for (ix, m) in maps.iter().enumerate() {
        println!("Map {ix}:\n{m}");
        println!("-------------------------------------------");
        let mut next_unmapped = unmapped
            .iter()
            .flat_map(|&r| m.map_interval(r).into_iter())
            .collect::<Vec<_>>();
        next_unmapped.sort_by(|a, b| a.min.partial_cmp(&b.min).unwrap());
        println!("Map {ix}: output = {:?}", next_unmapped);
        unmapped = next_unmapped;
    }

    let min_interval = unmapped.first().unwrap();
    println!("{min_interval}");
    unmapped.first().map(|i| i.min).unwrap()
}
