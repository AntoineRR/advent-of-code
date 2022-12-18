use std::{
    collections::{HashSet, VecDeque},
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

#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord, Debug)]
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

fn is_outside_bounds(cube: &Position, min_coords: &Position, max_coords: &Position) -> bool {
    cube.x < min_coords.x
        || cube.y < min_coords.y
        || cube.z < min_coords.z
        || cube.x > max_coords.x
        || cube.y > max_coords.y
        || cube.z > max_coords.z
}

fn get_trapped_air(
    origin: &Position,
    faces: &[Position],
    cubes: &HashSet<Position>,
    already_untrapped: &HashSet<Position>,
    min_coords: &Position,
    max_coords: &Position,
) -> (HashSet<Position>, bool) {
    let mut to_visit = VecDeque::new();
    to_visit.push_back((origin.clone(), 0));
    let mut visited = HashSet::new();
    visited.insert(origin.clone());
    let mut trapped = true;
    'general: while !to_visit.is_empty() {
        let (current, depth) = to_visit.pop_front().unwrap();
        for face in faces {
            let next = current + face.clone();
            if already_untrapped.contains(&next) || is_outside_bounds(&next, min_coords, max_coords)
            {
                trapped = false;
                break 'general;
            }
            if !visited.contains(&next) && !cubes.contains(&next) {
                visited.insert(next);
                to_visit.push_back((next, depth + 1));
            }
        }
    }
    (visited, trapped)
}

fn part_two(raw_data: &str) {
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
    let max_coords = Position::new(
        cubes.iter().map(|c| c.x).max().unwrap(),
        cubes.iter().map(|c| c.y).max().unwrap(),
        cubes.iter().map(|c| c.z).max().unwrap(),
    );
    let min_coords = Position::new(
        cubes.iter().map(|c| c.x).min().unwrap(),
        cubes.iter().map(|c| c.y).min().unwrap(),
        cubes.iter().map(|c| c.z).min().unwrap(),
    );
    let faces = vec![
        Position::new(-1, 0, 0),
        Position::new(1, 0, 0),
        Position::new(0, -1, 0),
        Position::new(0, 1, 0),
        Position::new(0, 0, -1),
        Position::new(0, 0, 1),
    ];
    let mut trapped: HashSet<Position> = HashSet::new();
    let mut untrapped: HashSet<Position> = HashSet::new();
    let mut result = 0;
    for cube in &cubes {
        let mut uncovered = 6;
        for face in faces.clone() {
            let current = cube.clone() + face;
            if cubes.contains(&current) || trapped.contains(&current) {
                uncovered -= 1;
            } else if !untrapped.contains(&current)
                && !cubes.contains(&current)
                && !is_outside_bounds(&current, &min_coords, &max_coords)
            {
                let (air, is_trapped) = get_trapped_air(
                    &current,
                    &faces,
                    &cubes,
                    &untrapped,
                    &min_coords,
                    &max_coords,
                );
                if is_trapped {
                    trapped.extend(air.iter());
                    uncovered -= 1;
                } else {
                    untrapped.extend(air.iter());
                }
            }
        }
        result += uncovered;
    }
    println!("Part 2 result is {result}")
}
