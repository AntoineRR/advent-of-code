use std::{error::Error, fmt::Display, fs};

fn main() {
    let data = get_input().unwrap();
    part_one(&data);
    part_two(&data);
}

fn get_input() -> Result<String, Box<dyn Error>> {
    let data = fs::read_to_string("./data/data.txt")?;
    Ok(data)
}

type Stack = Vec<char>;

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    n: usize,
}

impl Move {
    fn execute(&self, stacks: &mut [Stack]) {
        for _ in 0..self.n {
            let to_move = stacks[self.from - 1].pop().unwrap();
            stacks[self.to - 1].push(to_move);
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "move {} from {} to {}", self.n, self.from, self.to)
    }
}

fn parse_input(raw_data: &str) -> (Vec<Stack>, Vec<Move>) {
    let lines: Vec<&str> = raw_data.split("\n").collect();
    let mut stacks = vec![];
    for _ in 0..9 {
        stacks.push(Stack::new());
    }
    let mut moves = vec![];
    for line in lines[..8].iter().rev() {
        for (i, index) in (1..37).step_by(4).enumerate() {
            let letter = line.chars().nth(index).unwrap();
            if letter != ' ' {
                stacks[i].push(letter);
            }
        }
    }
    for line in &lines[10..] {
        let mut data = line.split(' ').filter_map(|s| s.parse::<usize>().ok());
        moves.push(Move {
            n: data.next().unwrap(),
            from: data.next().unwrap(),
            to: data.next().unwrap(),
        });
    }
    (stacks, moves)
}

fn display_stacks(stacks: &[Stack]) {
    let max_len = stacks.iter().map(|s| s.len()).max().unwrap();
    for i in (0..max_len).rev() {
        for stack in stacks {
            if stack.len() > i {
                print!("[{}] ", stack[i]);
            } else {
                print!("    ");
            }
        }
        println!("");
    }
    println!("");
}

fn part_one(raw_data: &str) {
    let parsed = parse_input(raw_data);
    if let (mut stacks, moves) = parsed {
        for crate_move in moves {
            println!("{crate_move}");
            display_stacks(&stacks);
            crate_move.execute(&mut stacks);
        }
        display_stacks(&stacks);
        let mut result = "".to_string();
        for stack in &stacks {
            result += &stack[stack.len() - 1].to_string();
        }
        println!("Part 1 result is {result}")
    }
}

fn part_two(raw_data: &str) {}
