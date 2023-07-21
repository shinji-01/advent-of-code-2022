use std::io::{self};

mod round;
mod r_move;
mod outcome;

fn main() -> Result<(), io::Error> {
    let mut total_sum = 0;

    for round in include_str!("input.txt")
        .lines()
        .map(|line| line.parse::<round::Round>()) {
        total_sum += round?.score();   
    }

    println!("Total: {:?}", total_sum);
    Ok(())
}
