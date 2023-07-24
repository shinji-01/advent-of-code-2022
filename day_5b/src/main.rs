#[derive(Debug, Clone)]
struct Stack {
    items: Vec<char>,
}

impl Stack {
    fn new(chars: Vec<char>) -> Self {
        Stack {
            items: chars,
        }
    }
}

fn main() {
    let mut stacks: Vec<Stack> = vec![
        Stack::new((['S', 'P', 'W', 'N', 'J', 'Z']).into_iter().rev().collect::<Vec<char>>()),
        Stack::new((['T', 'S', 'G']).into_iter().rev().collect::<Vec<char>>()),
        Stack::new((['H', 'L', 'R', 'Q', 'V']).into_iter().rev().collect::<Vec<char>>()),
        Stack::new((['D', 'T', 'S', 'V']).into_iter().rev().collect::<Vec<char>>()),
        Stack::new((['J', 'M', 'B', 'D', 'T', 'Z', 'Q']).into_iter().rev().collect::<Vec<char>>()),
        Stack::new((['L', 'Z', 'C', 'D', 'J', 'T', 'W', 'M']).into_iter().rev().collect::<Vec<char>>()),
        Stack::new((['J', 'T', 'G', 'W', 'M', 'P', 'L']).into_iter().rev().collect::<Vec<char>>()),
        Stack::new((['H', 'Q', 'F', 'B', 'T', 'M', 'G', 'N']).into_iter().rev().collect::<Vec<char>>()),
        Stack::new((['W', 'Q', 'B', 'P', 'C', 'G', 'D', 'R']).into_iter().rev().collect::<Vec<char>>()),
    ];

    include_str!("input.txt").lines().for_each(|line| {
        let (count, src, dst) = parse_instruction(line);
        println!("{:?}, {:?}, {:?}", count, src, dst);
        move_s(&mut stacks, count, src, dst);
    });


    for stack in stacks {
        println!("{:?}", stack);
    }
}

fn parse_instruction(instruction: &str) -> (usize, usize, usize){
    let mut p = instruction.split(' ');
    let (_, count, _, src, _, dst) = (p.next(), p.next(), p.next(), p.next(), p.next(), p.next());

    println!("{:?}, {:?}, {:?}", count, src, dst);
    (count.unwrap().parse::<usize>().unwrap(), src.unwrap().parse::<usize>().unwrap(), dst.unwrap().parse::<usize>().unwrap())
}

fn move_s(stacks: &mut [Stack], count: usize, src: usize, dst: usize) {
    let c = stacks[src-1].items.len()-count;
    let mut chunk: Vec<char> = stacks[src-1].items.drain(c..).collect();
    stacks[dst-1].items.append(&mut chunk);
}
