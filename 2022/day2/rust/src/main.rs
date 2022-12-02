use std::{collections::HashMap, error::Error, fs};

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
    let mut choice_map = HashMap::new();
    choice_map.insert("A", "X");
    choice_map.insert("B", "Y");
    choice_map.insert("C", "Z");
    let winning_situations = [&["A", "Y"], &["B", "Z"], &["C", "X"]];
    let mut point_map = HashMap::new();
    point_map.insert("X", 1);
    point_map.insert("Y", 2);
    point_map.insert("Z", 3);
    let result: i32 = lines
        .iter()
        .map(|l| {
            let round = l.split(" ").map(|c| c.trim()).collect::<Vec<&str>>();
            let mut score = 0;
            if winning_situations.contains(&round.as_slice().try_into().unwrap()) {
                score += 6;
            } else if choice_map.get(round[0]).unwrap() == &round[1] {
                score += 3;
            }
            score += point_map.get(round[1]).unwrap();
            score
        })
        .sum();
    println!("Part 1 result is {result}")
}

fn part_two(raw_data: &str) {}
