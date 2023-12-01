fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let digits: Vec<char> = l.chars().filter(|c| c.is_ascii_digit()).collect();
            let mut s = String::new();
            s.push(*digits.first().unwrap());
            s.push(*digits.last().unwrap());
            s
        })
        .map(|s: String| s.parse::<u32>().unwrap())
        .sum()
}

fn find_next_digit(input: &str) -> (Option<char>, Option<&str>) {
    if input.is_empty() {
        (None, None)
    } else if input.chars().next().unwrap().is_ascii_digit() {
        (Some(input.chars().next().unwrap()), Some(&input[1..]))
    } else if input.starts_with("one") {
        (Some('1'), Some(&input[2..]))
    } else if input.starts_with("two") {
        (Some('2'), Some(&input[2..]))
    } else if input.starts_with("three") {
        (Some('3'), Some(&input[4..]))
    } else if input.starts_with("four") {
        (Some('4'), Some(&input[3..]))
    } else if input.starts_with("five") {
        (Some('5'), Some(&input[4..]))
    } else if input.starts_with("six") {
        (Some('6'), Some(&input[2..]))
    } else if input.starts_with("seven") {
        (Some('7'), Some(&input[4..]))
    } else if input.starts_with("eight") {
        (Some('8'), Some(&input[4..]))
    } else if input.starts_with("nine") {
        (Some('9'), Some(&input[3..]))
    } else {
        (None, Some(&input[1..]))
    }
}

fn parse_line(input: &str) -> Vec<char> {
    let mut digits = Vec::new();
    let mut next = find_next_digit(input);
    if next.0.is_some() {
        digits.push(next.0.unwrap());
    }
    while next.1.is_some() {
        next = find_next_digit(next.1.unwrap());
        if next.0.is_some() {
            digits.push(next.0.unwrap());
        }
    }
    print!("{} -> {:?}", input, digits);
    digits
}

fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(parse_line)
        .map(|v| {
            let mut s = String::new();
            s.push(*v.first().unwrap());
            s.push(*v.last().unwrap());
            let num = s.parse::<u32>().unwrap();
            println!(" -> {}", num);
            num
        })
        .sum()
}

fn main() {
    let input = include_str!("../data/dec1.txt");
    let sum = part_two(input);
    println!("{sum}");
}

#[cfg(test)]
mod tests {
    use super::find_next_digit;

    #[test]
    fn test_find_next_digit() {
        let (d, i) = find_next_digit("one34");
        assert_eq!(d, Some('1'));
        println!("{:?} {:?}", d, i);
        let (n, j) = find_next_digit(i.unwrap());
        assert_eq!(n, None);
        println!("{:?} {:?}", n, j);
        let (d2, i2) = find_next_digit(j.unwrap());
        assert_eq!(d2, Some('3'));
        println!("{:?} {:?}", d2, i2);
        // let (d3, i3) = find_next_digit(i2.unwrap());
        // assert_eq!(d3, Some(4));
        // let (d4, i4) = find_next_digit(i3.unwrap());
        // assert_eq!(d4, None);
        // assert_eq!(i4, None);
    }
}
