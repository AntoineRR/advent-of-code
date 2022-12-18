use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
    ops::Add,
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

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl TryFrom<Vec<i32>> for Position {
    type Error = ();

    fn try_from(value: Vec<i32>) -> Result<Self, Self::Error> {
        if value.len() < 3 {
            return Err(());
        }
        Ok(Self {
            x: value[0],
            y: value[1],
            z: value[2],
        })
    }
}

impl Add<Position> for Position {
    type Output = Self;

    fn add(self, rhs: Position) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Position {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

fn part_one(raw_data: &str) {
    let lines: Vec<&str> = raw_data.split("\n").collect();
    let cubes: HashSet<Position> = lines
        .iter()
        .map(|l| {
            l.split(',')
                .map(|c| c.parse().unwrap())
                .collect::<Vec<i32>>()
                .try_into()
                .unwrap()
        })
        .collect();
    let faces = vec![
        Position::new(-1, 0, 0),
        Position::new(1, 0, 0),
        Position::new(0, -1, 0),
        Position::new(0, 1, 0),
        Position::new(0, 0, -1),
        Position::new(0, 0, 1),
    ];
    let mut result = 0;
    for cube in &cubes {
        let mut uncovered = 6;
        for face in faces.clone() {
            if cubes.contains(&(cube.clone() + face)) {
                uncovered -= 1;
            }
        }
        result += uncovered;
    }
    println!("Part 1 result is {result}")
}

fn part_two(raw_data: &str) {}
