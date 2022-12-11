use std::{error::Error, fs};

fn main() {
    let data = get_input().unwrap();
    part_one(&data);
    part_two(&data);
}

fn get_input() -> Result<String, Box<dyn Error>> {
    let data = fs::read_to_string("./data/data.txt")?;
    Ok(data)
}

#[derive(Debug)]
enum Operation {
    MultiplyOldBy(u64),
    AddToOld(u64),
    MultiplyOldByItself,
}

impl Operation {
    fn from_str(raw: &str) -> Self {
        let (op, n) = raw.split_once(' ').unwrap();
        if op == "+" {
            Self::AddToOld(n.parse().unwrap())
        } else {
            match n.parse() {
                Ok(value) => Self::MultiplyOldBy(value),
                Err(_) => Self::MultiplyOldByItself,
            }
        }
    }

    fn apply(&self, worry_level: &mut u64) {
        match self {
            Self::AddToOld(n) => *worry_level += n,
            Self::MultiplyOldBy(n) => *worry_level *= n,
            Self::MultiplyOldByItself => *worry_level *= *worry_level,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    number: u32,
    items: Vec<u64>,
    operation: Operation,
    divisible_by_test: u64,
    true_throw_to: usize,
    false_throw_to: usize,
}

fn get_monkeys(raw_data: &str) -> Vec<Monkey> {
    let raw_monkeys: Vec<&str> = raw_data.split("\n\n").collect();
    let mut monkeys = vec![];
    for raw_monkey in raw_monkeys {
        let mut lines = raw_monkey.split('\n');
        let number = lines
            .next()
            .unwrap()
            .chars()
            .nth(7)
            .unwrap()
            .to_digit(10)
            .unwrap();
        let items = lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|i| i.parse().unwrap())
            .collect();
        let operation = Operation::from_str(lines.next().unwrap().split_once("old ").unwrap().1);
        let divisible_by_test = lines
            .next()
            .unwrap()
            .split_once("by ")
            .unwrap()
            .1
            .parse()
            .unwrap();
        let true_throw_to = lines
            .next()
            .unwrap()
            .chars()
            .last()
            .unwrap()
            .to_digit(10)
            .unwrap() as usize;
        let false_throw_to = lines
            .next()
            .unwrap()
            .chars()
            .last()
            .unwrap()
            .to_digit(10)
            .unwrap() as usize;
        monkeys.push(Monkey {
            number,
            items,
            operation,
            divisible_by_test,
            true_throw_to,
            false_throw_to,
        })
    }
    monkeys
}

fn part_one(raw_data: &str) {
    let mut monkeys = get_monkeys(raw_data);
    let mut monkey_businesses = vec![0; 8];
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            for j in 0..monkeys[i].items.len() {
                let mut worry_level = monkeys[i].items[j];
                monkeys[i].operation.apply(&mut worry_level);
                worry_level /= 3;
                let to = if worry_level % monkeys[i].divisible_by_test == 0 {
                    monkeys[i].true_throw_to
                } else {
                    monkeys[i].false_throw_to
                };
                monkeys[to].items.push(worry_level);
                monkey_businesses[i] += 1;
            }
            monkeys[i].items.clear();
        }
    }
    monkey_businesses.sort_by(|a, b| b.cmp(a));
    let result = monkey_businesses[0] * monkey_businesses[1];
    println!("Part 1 result is {result}")
}

fn part_two(raw_data: &str) {}
