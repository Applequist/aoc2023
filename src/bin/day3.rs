use aoc2023::day3::parse_line;

fn main() {
    let input = include_str!("../../data/dec3.txt");
    let indexes: Vec<_> = input.lines().map(parse_line).collect();
    let mut sum = 0;

    // First line
    let mut all_sym = indexes[0].symbols.clone();
    all_sym.extend(&indexes[1].symbols);
    sum += indexes[0].part_numbers(&all_sym).sum::<u32>();

    for ix in 1..indexes.len() - 1 {
        let mut all_sym = indexes[ix - 1].symbols.clone();
        all_sym.extend(&indexes[ix].symbols);
        all_sym.extend(&indexes[ix + 1].symbols);
        sum += indexes[ix].part_numbers(&all_sym).sum::<u32>();
    }

    // Last line
    let mut all_sym = indexes[indexes.len() - 2].symbols.clone();
    all_sym.extend(&indexes[indexes.len() - 1].symbols);
    sum += indexes[indexes.len() - 1]
        .part_numbers(&all_sym)
        .sum::<u32>();

    println!("Answer for Day 3");
    println!("\tPart 1: {sum}");
}
