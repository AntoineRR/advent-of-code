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
    let point_map: HashMap<char, usize> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .char_indices()
        .map(|(i, c)| (c, i + 1))
        .collect();
    let result: usize = lines
        .iter()
        .map(|line| {
            let mut encountered_char = HashSet::new();
            let n = line.len();
            let mut found = false;
            let a = line
                .char_indices()
                .map(|(i, c)| {
                    if i < n / 2 {
                        encountered_char.insert(c);
                        0
                    } else if encountered_char.contains(&c) && !found {
                        found = true;
                        return *point_map.get(&c).unwrap();
                    } else {
                        0
                    }
                })
                .sum::<usize>();
            println!("{a}");
            a
        })
        .sum();
    println!("Part 1 result is {result}")
}

fn part_two(raw_data: &str) {}
