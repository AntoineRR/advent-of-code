use std::{cmp::Ordering, error::Error, fs};

fn main() {
    let data = get_input().unwrap();
    part_one(&data);
    part_two(&data);
}

fn get_input() -> Result<String, Box<dyn Error>> {
    let data = fs::read_to_string("./data/data.txt")?;
    Ok(data)
}

fn parse_as_vec(input: &str) -> Option<Vec<&str>> {
    if !input.starts_with('[') {
        return None;
    }
    let mut result: Vec<&str> = vec![];
    let mut index = 1;
    while index < input.len() - 1 {
        if input.chars().nth(index).unwrap() == '[' {
            let mut count = 1;
            let start = index;
            while count != 0 {
                index += 1;
                if input.chars().nth(index).unwrap() == '[' {
                    count += 1;
                } else if input.chars().nth(index).unwrap() == ']' {
                    count -= 1;
                }
            }
            result.push(&input[start..index + 1]);
        }
        if let Some(_) = input.chars().nth(index).unwrap().to_digit(10) {
            let start = index;
            while let Some(_) = input.chars().nth(index).unwrap().to_digit(10) {
                index += 1;
            }
            result.push(&input[start..index]);
        }
        index += 1;
    }
    Some(result)
}

fn parse_as_int(input: &str) -> Option<usize> {
    input.parse().ok()
}

fn is_right_order(pair: &(&str, &str)) -> Option<bool> {
    if let Some(v1) = parse_as_vec(pair.0) {
        if let Some(v2) = parse_as_vec(pair.1) {
            let mut index = 0;
            while index < v1.len() && index < v2.len() {
                if let Some(r) = is_right_order(&(v1[index], v2[index])) {
                    return Some(r);
                }
                index += 1;
            }
            if index == v1.len() && index == v2.len() {
                return None;
            } else if index == v1.len() {
                return Some(true);
            } else {
                return Some(false);
            }
        } else if let Some(i2) = parse_as_int(pair.1) {
            let vecified_i2 = "[".to_string() + &i2.to_string() + "]";
            return is_right_order(&(pair.0, &vecified_i2));
        } else {
            panic!("Impossible parsing: {}", pair.1);
        }
    } else if let Some(i1) = parse_as_int(pair.0) {
        if let Some(_) = parse_as_vec(pair.1) {
            let vecified_i1 = "[".to_string() + &i1.to_string() + "]";
            return is_right_order(&(&vecified_i1, pair.1));
        } else if let Some(i2) = parse_as_int(pair.1) {
            if i1 == i2 {
                return None;
            } else if i1 < i2 {
                return Some(true);
            } else {
                return Some(false);
            }
        } else {
            panic!("Impossible parsing: {}", pair.1);
        }
    } else {
        panic!("Impossible parsing: {}", pair.0);
    }
}

fn part_one(raw_data: &str) {
    let pairs: Vec<(&str, &str)> = raw_data
        .split("\n\n")
        .map(|pair| pair.split_once('\n').unwrap())
        .collect();
    let mut result = 0;
    for (i, pair) in pairs.iter().enumerate() {
        if let Some(right) = is_right_order(pair) {
            if right {
                result += i + 1;
            }
        }
    }
    println!("Part 1 result is {result}")
}

fn compare(a: &str, b: &str) -> Ordering {
    if let Some(r) = is_right_order(&(a, b)) {
        if r {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    } else {
        return Ordering::Equal;
    }
}

fn part_two(raw_data: &str) {
    let mut packets: Vec<&str> = raw_data
        .split("\n\n")
        .flat_map(|pair| pair.split('\n'))
        .collect();
    packets.push("[[2]]");
    packets.push("[[6]]");
    packets.sort_by(|a, b| compare(a, b));
    let first = packets
        .iter()
        .enumerate()
        .find(|(_, &p)| p == "[[2]]")
        .map(|a| a.0 + 1)
        .unwrap();
    let second = packets
        .iter()
        .enumerate()
        .find(|(_, &p)| p == "[[6]]")
        .map(|a| a.0 + 1)
        .unwrap();
    let result = first * second;
    println!("Part 2 result is {result}")
}
