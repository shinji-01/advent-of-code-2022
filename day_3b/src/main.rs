fn to_priority(c: char) -> usize {
    match c {
        'a'..='z' => c as usize - 96,
        'A'..='Z' => c as usize - 38,
        _ => 0,
    }
}

fn main() {
    let lines = include_str!("input.txt").lines().collect::<Vec<&str>>();
    let chunks = lines.chunks(3);
    let response = chunks.map(|chunk| {
        let elf1 = chunk[0];
        let elf2 = chunk[1];
        let elf3 = chunk[2];

        let item = elf1.chars().filter(|c| elf2.find(*c).is_some() && elf3.find(*c).is_some()).next();
        to_priority(item.unwrap())
    }).sum::<usize>();

    println!("{response}");
}
