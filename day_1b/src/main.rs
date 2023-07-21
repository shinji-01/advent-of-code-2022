use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Elf {
    id: u32,
    calories: Vec<u32>
}

fn main() {
    if let Ok(lines) = read_lines("./src/input.txt") {
        let mut id = 1;

        let mut elf = Elf {
            id: id,
            calories: Vec::new(),
        };

        let mut elves: Vec<Elf> = Vec::new();

        for line in lines {
            if let Ok(line) = line {
                if line.is_empty() {
                    elves.push(elf);

                    id += 1;
                    elf = Elf {
                        id,
                        calories: Vec::new()
                    };
                } else {
                    let calories: u32 = line.trim().parse().unwrap();
                    elf.calories.push(calories);
                }
            }
        }

        let mut max: u32 = 0;

        //let sums = elves.iter().map(|elf| elf.calories.iter().sum::<u32>());
        let mut sums = elves.iter().map(|elf| elf.calories.iter().sum::<u32>()).collect::<Vec<u32>>();
        //for elf in elves {
        //    let total_calories: u32 = elf.calories.iter().sum::<u32>();
        //    if total_calories > max {
        //        max = total_calories
        //    }
        //}
        sums.sort();
        sums.reverse();
        println!("{:?}", (&sums[0..3]).iter().sum::<u32>());
    }
}

fn read_lines<P>(filename: P) -> Result<io::Lines<io::BufReader<File>>, io::Error>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
