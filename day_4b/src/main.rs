fn main() {
    let m = include_str!("input.txt").lines().filter(|line| {
        println!("{:?}", line);
        let mut pair = line.split(',');
        let (section1, section2) = (pair.next().unwrap(), pair.next().unwrap());
        let (mut section1_range, mut section2_range) = (section1.split('-'), section2.split('-'));
        let begin_s1: usize = section1_range.next().unwrap().parse().unwrap();
        let end_s1: usize = section1_range.next().unwrap().parse().unwrap();
        let begin_s2: usize = section2_range.next().unwrap().parse().unwrap();
        let end_s2: usize = section2_range.next().unwrap().parse().unwrap();

        if begin_s1 <= end_s2 && end_s1 >= begin_s2 {
            true
        } else {
            false
        }
    }).collect::<Vec<_>>();

    println!("{:?}", m.len());
}
