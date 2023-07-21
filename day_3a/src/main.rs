fn to_priority(c: char) -> usize {
    match c {
        'a'..='z' => c as usize - 96,
        'A'..='Z' => c as usize - 38,
        _ => 0,
    }
}

fn main() {
    let response = include_str!("input.txt").lines().map(|line| {
        let part1 = String::from(&line[..(line.len() / 2)]);
        let part2 = String::from(&line[(line.len() / 2)..]);

        let item = part1.chars().filter(|c| part2.find(*c).is_some()).next();
        to_priority(item.unwrap())
    }).sum::<usize>();

    println!("{:?}", response);
}
