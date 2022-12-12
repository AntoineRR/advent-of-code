use std::{
    collections::{HashMap, HashSet, VecDeque},
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

fn get_map(raw_data: &str) -> Vec<Vec<usize>> {
    let lines: Vec<&str> = raw_data.split("\n").collect();
    let height_mapping: HashMap<char, usize> = "abcdefghijklmnopqrstuvwxyz"
        .char_indices()
        .map(|(i, c)| (c, i))
        .collect();
    lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| {
                    if c == 'E' {
                        25
                    } else if c == 'S' {
                        0
                    } else {
                        *height_mapping.get(&c).unwrap()
                    }
                })
                .collect()
        })
        .collect()
}

type Position = (usize, usize);

#[derive(Default, Clone)]
struct Node {
    position: Position,
    parent: Option<Box<Node>>,
}

fn walk(
    raw_data: &str,
    map: &[Vec<usize>],
    end: Position,
    to_visit: &mut VecDeque<Node>,
    visited: &mut HashSet<Position>,
) -> usize {
    let mut end_node = Node::default();
    while !to_visit.is_empty() {
        let from_node = to_visit.pop_front().unwrap();
        let from = from_node.position;
        if from == end {
            end_node = from_node;
            break;
        }
        visited.insert(from);
        let current_height = map[from.0][from.1];
        let mut to_check = vec![];
        if from.0 > 0 {
            to_check.push((from.0 - 1, from.1));
        }
        if from.0 < map.len() - 1 {
            to_check.push((from.0 + 1, from.1));
        }
        if from.1 > 0 {
            to_check.push((from.0, from.1 - 1));
        }
        if from.1 < map[0].len() - 1 {
            to_check.push((from.0, from.1 + 1));
        }
        for pos in to_check {
            if !visited.contains(&pos) {
                let pos_height = map[pos.0][pos.1];
                if pos_height <= current_height + 1 {
                    to_visit.push_back(Node {
                        position: pos,
                        parent: Some(Box::new(from_node.clone())),
                    });
                    visited.insert(pos);
                }
            }
        }
        //display_visited(raw_data, visited);
    }
    let mut result = 0;
    while let Some(n) = end_node.parent {
        result += 1;
        end_node = *n;
    }
    result
}

fn get_start(raw_data: &str) -> Position {
    let (i, line) = raw_data
        .split("\n")
        .enumerate()
        .find(|l| l.1.contains('S'))
        .unwrap();
    let (j, _) = line.chars().enumerate().find(|c| c.1 == 'S').unwrap();
    (i, j)
}

fn get_end(raw_data: &str) -> Position {
    let (i, line) = raw_data
        .split("\n")
        .enumerate()
        .find(|l| l.1.contains('E'))
        .unwrap();
    let (j, _) = line.chars().enumerate().find(|c| c.1 == 'E').unwrap();
    (i, j)
}

fn display_visited(raw_data: &str, visited: &HashSet<Position>) {
    let map: Vec<Vec<char>> = raw_data.split('\n').map(|l| l.chars().collect()).collect();
    let mut result = "".to_string();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if visited.contains(&(i, j)) {
                result += "x";
            } else {
                result += &map[i][j].to_string();
            }
        }
        result += "\n";
    }
    println!("{}", result);
}

fn part_one(raw_data: &str) {
    let map = get_map(raw_data);
    let start = get_start(raw_data);
    let end = get_end(raw_data);
    let mut visited = HashSet::new();
    let mut to_visit = VecDeque::from([Node {
        position: start,
        parent: None,
    }]);
    let result = walk(raw_data, &map, end, &mut to_visit, &mut visited);
    println!("Part 1 result is {result}");
}

fn get_start_list(map: &[Vec<usize>]) -> Vec<Position> {
    let mut result = vec![];
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 0 {
                result.push((i, j));
            }
        }
    }
    result
}

fn part_two(raw_data: &str) {
    let map = get_map(raw_data);
    let starts = get_start_list(&map);
    let end = get_end(raw_data);
    let mut shortest = map.len() * map[0].len();
    for start in starts {
        let mut visited = HashSet::new();
        let mut to_visit = VecDeque::from([Node {
            position: start,
            parent: None,
        }]);
        let result = walk(raw_data, &map, end, &mut to_visit, &mut visited);
        if result < shortest && result != 0 {
            shortest = result;
        }
    }
    println!("Part 2 result is {shortest}");
}
