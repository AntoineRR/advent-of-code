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

fn display_pos(tail_pos: (i32, i32), head_pos: (i32, i32)) {
    let relative_pos = (tail_pos.0 - head_pos.0, tail_pos.1 - head_pos.1);
    if relative_pos.0 > 1 || relative_pos.0 < -1 || relative_pos.1 > 1 || relative_pos.1 < -1 {
        println!("no");
    }
    let mut to_print = "".to_string();
    for i in -1..2 {
        for j in -1..2 {
            if relative_pos.0 == i && relative_pos.1 == j {
                to_print += "T";
            } else if i == 0 && j == 0 {
                to_print += "H";
            } else {
                to_print += " ";
            }
        }
        to_print += "\n";
    }
    println!("{to_print}");
}

fn part_one(raw_data: &str) {
    let lines: Vec<&str> = raw_data.split("\n").collect();
    let mut visited = HashSet::new();
    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);
    for line in lines {
        let (dir, amount) = line.split_once(' ').unwrap();
        let amount = amount.parse().unwrap();
        for _ in 0..amount {
            if dir == "R" {
                head_pos.1 += 1;
            } else if dir == "U" {
                head_pos.0 -= 1;
            } else if dir == "L" {
                head_pos.1 -= 1;
            } else {
                head_pos.0 += 1;
            }
            move_tail(head_pos, &mut tail_pos);
            if !visited.contains(&tail_pos) {
                visited.insert(tail_pos);
            }
            // println!("{}, head: {:?}, tail: {:?}", line, head_pos, tail_pos);
            // display_pos(tail_pos, head_pos);
        }
    }
    let result = visited.len();
    println!("Part 1 result is {result}")
}

fn move_tail(head_pos: (i32, i32), tail_pos: &mut (i32, i32)) {
    if head_pos.0 - tail_pos.0 >= 2 {
        tail_pos.0 += 1;
        if head_pos.1 - tail_pos.1 >= 1 {
            tail_pos.1 += 1;
        } else if head_pos.1 - tail_pos.1 <= -1 {
            tail_pos.1 -= 1;
        }
    } else if head_pos.0 - tail_pos.0 <= -2 {
        tail_pos.0 -= 1;
        if head_pos.1 - tail_pos.1 >= 1 {
            tail_pos.1 += 1;
        } else if head_pos.1 - tail_pos.1 <= -1 {
            tail_pos.1 -= 1;
        }
    }
    if head_pos.1 - tail_pos.1 >= 2 {
        tail_pos.1 += 1;
        if head_pos.0 - tail_pos.0 >= 1 {
            tail_pos.0 += 1;
        } else if head_pos.0 - tail_pos.0 <= -1 {
            tail_pos.0 -= 1;
        }
    } else if head_pos.1 - tail_pos.1 <= -2 {
        tail_pos.1 -= 1;
        if head_pos.0 - tail_pos.0 >= 1 {
            tail_pos.0 += 1;
        } else if head_pos.0 - tail_pos.0 <= -1 {
            tail_pos.0 -= 1;
        }
    }
}

fn part_two(raw_data: &str) {
    let lines: Vec<&str> = raw_data.split("\n").collect();
    let mut visited = HashSet::new();
    let mut knot_pos = vec![(0, 0); 10];
    for line in lines {
        let (dir, amount) = line.split_once(' ').unwrap();
        let amount = amount.parse().unwrap();
        for _ in 0..amount {
            if dir == "R" {
                knot_pos[0].1 += 1;
            } else if dir == "U" {
                knot_pos[0].0 -= 1;
            } else if dir == "L" {
                knot_pos[0].1 -= 1;
            } else {
                knot_pos[0].0 += 1;
            }
            for i in 0..9 {
                move_tail(knot_pos[i], &mut knot_pos[i + 1]);
            }

            if !visited.contains(&knot_pos[9]) {
                visited.insert(knot_pos[9]);
            }
            // println!("{}, head: {:?}, tail: {:?}", line, head_pos, tail_pos);
            // display_pos(tail_pos, head_pos);
        }
    }
    let result = visited.len();
    println!("Part 2 result is {result}")
}
