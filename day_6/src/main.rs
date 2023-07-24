const CHUNK_SIZE: usize = 4;

// Part 2
// const CHUNK_SIZE: usize = 14;

fn main() {
    let segment = include_str!("input.txt");

    for n in 0..(segment.len() - CHUNK_SIZE) {
        let chunk: Vec<char> = (&segment[n..n + CHUNK_SIZE]).chars().collect();
        let res = is_unique(chunk);

        if res {
            println!("result: {}", n + CHUNK_SIZE);
            break;
        }
    }
}

fn is_unique(chunk: Vec<char>) -> bool {
    for i in 0..chunk.len() {
        for j in 0..chunk.len() {
            if j == i {
                continue;
            }
            if chunk[i] == chunk[j] {
                return false;
            }
        }
    }

    true
}
