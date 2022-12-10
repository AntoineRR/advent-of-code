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

fn inc_cycles(cycles: &mut i32, useful_cycles: &[i32], x: i32, signal_strength_added: &mut i32) {
    *cycles += 1;
    if useful_cycles.contains(cycles) {
        *signal_strength_added += *cycles * x;
    }
}

fn part_one(raw_data: &str) {
    let lines: Vec<&str> = raw_data.split("\n").collect();
    let mut x = 1;
    let mut cycles = 0;
    let useful_cycles = vec![20, 60, 100, 140, 180, 220];
    let mut result = 0;
    for line in lines {
        if line.contains("noop") {
            inc_cycles(&mut cycles, &useful_cycles, x, &mut result);
        } else {
            let (_, to_add) = line.split_once(' ').unwrap();
            let to_add: i32 = to_add.parse().unwrap();
            inc_cycles(&mut cycles, &useful_cycles, x, &mut result);
            inc_cycles(&mut cycles, &useful_cycles, x, &mut result);
            x += to_add;
        }
    }
    println!("Part 1 result is {result}");
}

fn get_char_to_draw(cycles: i32, x: i32) -> char {
    if (cycles % 40) - x <= 1 && (cycles % 40) - x >= -1 {
        '#'
    } else {
        '.'
    }
}

fn part_two(raw_data: &str) {
    let lines: Vec<&str> = raw_data.split("\n").collect();
    let mut x = 1;
    let mut cycles = 0;
    let mut result = "".to_string();
    for line in lines {
        if line.contains("noop") {
            result += &get_char_to_draw(cycles, x).to_string();
            cycles += 1;
        } else {
            let (_, to_add) = line.split_once(' ').unwrap();
            let to_add: i32 = to_add.parse().unwrap();
            result += &get_char_to_draw(cycles, x).to_string();
            cycles += 1;
            if cycles % 40 == 0 {
                result += "\n";
            }
            result += &get_char_to_draw(cycles, x).to_string();
            cycles += 1;
            x += to_add;
        }
        if cycles % 40 == 0 {
            result += "\n";
        }
    }
    println!("Part 2 result is:\n{result}");
}
