use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
};

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
    let mut rocks = HashSet::new();
    for line in lines {
        let coordinates: Vec<(i32, i32)> = line
            .split(" -> ")
            .map(|c| {
                c.split_once(',')
                    .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                    .unwrap()
            })
            .collect();
        let mut to_add = HashSet::new();
        for i in 0..coordinates.len() - 1 {
            let start = coordinates[i];
            let end = coordinates[i + 1];
            if start.0 == end.0 {
                if start.1 < end.1 {
                    for j in start.1..end.1 + 1 {
                        to_add.insert((start.0, j));
                    }
                } else {
                    for j in end.1..start.1 + 1 {
                        to_add.insert((start.0, j));
                    }
                }
            } else if start.1 == end.1 {
                if start.0 < end.0 {
                    for j in start.0..end.0 + 1 {
                        to_add.insert((j, start.1));
                    }
                } else {
                    for j in end.0..start.0 + 1 {
                        to_add.insert((j, start.1));
                    }
                }
            } else {
                panic!("Invalid rocks");
            }
        }
        rocks.extend(to_add);
    }
    let lowest_x = *rocks.iter().map(|(x, _)| x).min().unwrap();
    let highest_x = *rocks.iter().map(|(x, _)| x).max().unwrap();
    let highest_y = *rocks.iter().map(|(_, y)| y).max().unwrap();

    let origin = (500, 0);
    let mut resting_sand = HashSet::new();

    let display = |resting_sand: &HashSet<(i32, i32)>| {
        let mut to_display = "".to_string();
        for i in 0..highest_y + 1 {
            for j in lowest_x..highest_x + 1 {
                if rocks.contains(&(j, i)) {
                    to_display += "#";
                } else if resting_sand.contains(&(j, i)) {
                    to_display += "o";
                } else {
                    to_display += " ";
                }
            }
            to_display += "\n";
        }
        println!("{to_display}");
    };

    'general: loop {
        let mut current = origin;
        loop {
            let below = (current.0, current.1 + 1);
            if rocks.contains(&below) || resting_sand.contains(&below) {
                let below_left = (current.0 - 1, current.1 + 1);
                if rocks.contains(&below_left) || resting_sand.contains(&below_left) {
                    let below_right = (current.0 + 1, current.1 + 1);
                    if rocks.contains(&below_right) || resting_sand.contains(&below_right) {
                        resting_sand.insert(current);
                        break;
                    } else {
                        current = below_right;
                    }
                } else {
                    current = below_left;
                }
            } else {
                current = below;
            }
            if current.0 > highest_x || current.0 < lowest_x || current.1 > highest_y {
                break 'general;
            }
            // display(&resting_sand);
        }
    }
    let result = resting_sand.len();
    println!("Part 1 result is {result}")
}

fn part_two(raw_data: &str) {}
