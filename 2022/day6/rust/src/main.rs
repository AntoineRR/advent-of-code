use std::{collections::HashSet, error::Error, fs};

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
    let line = lines[0];
    let result = line
        .as_bytes()
        .windows(4)
        .enumerate()
        .skip_while(|chunk| {
            chunk.1.len() != HashSet::<u8>::from_iter(chunk.1.iter().cloned()).len()
        })
        .next()
        .unwrap()
        .0
        + 4;
    println!("Part 1 result is {result}")
}

fn part_two(raw_data: &str) {
    let lines: Vec<&str> = raw_data.split("\n").collect();
    let line = lines[0];
    let result = line
        .as_bytes()
        .windows(14)
        .enumerate()
        .skip_while(|chunk| {
            chunk.1.len() != HashSet::<u8>::from_iter(chunk.1.iter().cloned()).len()
        })
        .next()
        .unwrap()
        .0
        + 14;
    println!("Part 2 result is {result}")
}
