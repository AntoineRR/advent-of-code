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

fn part_two(raw_data: &str) {
    let lines: Vec<&str> = raw_data.split("\n").collect();
    let mut x_choice_map = HashMap::new();
    x_choice_map.insert("A", "C");
    x_choice_map.insert("B", "A");
    x_choice_map.insert("C", "B");
    let mut y_choice_map = HashMap::new();
    y_choice_map.insert("A", "A");
    y_choice_map.insert("B", "B");
    y_choice_map.insert("C", "C");
    let mut z_choice_map = HashMap::new();
    z_choice_map.insert("A", "B");
    z_choice_map.insert("B", "C");
    z_choice_map.insert("C", "A");
    let mut choice_map = HashMap::new();
    choice_map.insert("X", x_choice_map);
    choice_map.insert("Y", y_choice_map);
    choice_map.insert("Z", z_choice_map);
    let mut choice_point_map = HashMap::new();
    choice_point_map.insert("A", 1);
    choice_point_map.insert("B", 2);
    choice_point_map.insert("C", 3);
    let mut success_point_map = HashMap::new();
    success_point_map.insert("X", 0);
    success_point_map.insert("Y", 3);
    success_point_map.insert("Z", 6);
    let result: i32 = lines
        .iter()
        .map(|l| {
            let round = l.split(" ").map(|c| c.trim()).collect::<Vec<&str>>();
            let mut score = 0;
            score += choice_point_map
                .get(choice_map.get(round[1]).unwrap().get(round[0]).unwrap())
                .unwrap();
            score += success_point_map.get(round[1]).unwrap();
            score
        })
        .sum();
    println!("Part 2 result is {result}")
}
