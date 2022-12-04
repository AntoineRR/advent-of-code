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
    let result: i32 = lines
        .iter()
        .map(|line| {
            let assignments: Vec<Vec<i32>> = line
                .split(',')
                .map(|elf| elf.split('-').map(|x| x.parse().unwrap()).collect())
                .collect();
            if (assignments[0][0] <= assignments[1][0] && assignments[0][1] >= assignments[1][1])
                || (assignments[1][0] <= assignments[0][0]
                    && assignments[1][1] >= assignments[0][1])
            {
                1
            } else {
                0
            }
        })
        .sum();
    println!("Part 1 result is {result}");
}

fn part_two(raw_data: &str) {}
