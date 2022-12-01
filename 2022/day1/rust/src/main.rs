use std::{error::Error, fs};

fn main() {
    let data = get_input().unwrap();
    let lines: Vec<&str> = data.split("\n").collect();
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

fn get_input() -> Result<String, Box<dyn Error>> {
    let data = fs::read_to_string("./data/data.txt")?;
    Ok(data)
}
