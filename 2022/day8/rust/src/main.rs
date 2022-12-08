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

fn parse_matrix(raw_data: &str) -> Vec<Vec<u32>> {
    raw_data
        .split("\n")
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn part_one(raw_data: &str) {
    let matrix = parse_matrix(raw_data);
    let n = matrix.len();
    let m = matrix[0].len();
    let mut result = n * 2 + m * 2 - 4; // outer ring
    let mut visible_trees = HashSet::new();

    // First go from top left to bottom right
    // This gives tree visible from left or top
    let mut max_on_lines = vec![];
    for i in 0..n {
        max_on_lines.push(matrix[i][0]);
    }
    let mut max_on_columns = vec![];
    for j in 0..m {
        max_on_columns.push(matrix[0][j]);
    }
    for i in 1..n - 1 {
        for j in 1..m - 1 {
            let current = matrix[i][j];
            if current > max_on_lines[i] || current > max_on_columns[j] {
                result += 1;
                visible_trees.insert((i, j));
            }
            if current > max_on_lines[i] {
                max_on_lines[i] = current;
            }
            if current > max_on_columns[j] {
                max_on_columns[j] = current;
            }
        }
    }

    // Then go from bottom right to top left
    // This gives tree visible from right or bottom
    let mut max_on_lines = vec![];
    for i in 0..n {
        max_on_lines.push(matrix[i][m - 1]);
    }
    let mut max_on_columns = vec![];
    for j in 0..m {
        max_on_columns.push(matrix[n - 1][j]);
    }
    for i in (1..n - 1).rev() {
        for j in (1..m - 1).rev() {
            let current = matrix[i][j];
            if (current > max_on_lines[i] || current > max_on_columns[j])
                && !visible_trees.contains(&(i, j))
            {
                result += 1;
                visible_trees.insert((i, j));
            }
            if current > max_on_lines[i] {
                max_on_lines[i] = current;
            }
            if current > max_on_columns[j] {
                max_on_columns[j] = current;
            }
        }
    }

    println!("Part 1 result is {result}")
}

fn part_two(raw_data: &str) {}
