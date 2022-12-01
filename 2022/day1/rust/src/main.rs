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

fn part_one(raw_data: &str) {
    let lines: Vec<&str> = raw_data.split("\n").collect();
    let mut max = 0;
    let mut current_sum = 0;
    for line in lines {
        if line.is_empty() {
            if current_sum > max {
                max = current_sum;
            }
            current_sum = 0;
            continue;
        }
        current_sum += line.parse::<i32>().unwrap();
    }
    println!("Result is {max}");
}

fn part_two(raw_data: &str) {
    let lines: Vec<&str> = raw_data.split("\n").collect();
    let mut sums = vec![];
    let mut current_sum = 0;
    for line in lines {
        if line.is_empty() {
            sums.push(current_sum);
            current_sum = 0;
            continue;
        }
        current_sum += line.parse::<i32>().unwrap();
    }
    sums.sort_unstable_by(|a, b| b.cmp(a));
    let result: i32 = sums.iter().take(3).sum();
    println!("Result is {result}");
}
