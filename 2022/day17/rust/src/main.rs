use std::{collections::HashSet, error::Error, fmt::Display, fs};

fn main() {
    let data = get_input().unwrap();
    part_one(&data);
    part_two(&data);
}

fn get_input() -> Result<String, Box<dyn Error>> {
    let data = fs::read_to_string("./data/data.txt")?;
    Ok(data)
}

#[derive(Clone)]
enum Shift {
    Left,
    Right,
}

fn get_shift_sequence(raw_data: &str) -> Vec<Shift> {
    raw_data
        .chars()
        .map(|c| if c == '>' { Shift::Right } else { Shift::Left })
        .collect()
}

enum Direction {
    Left,
    Right,
    Down,
}

type Position = (i32, i32);

#[derive(Debug)]
struct MoveError {}

impl Display for MoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot move rock")
    }
}

#[derive(Clone)]
struct Rock {
    cells: Vec<Position>,
    width: i32,
    height: i32,
}

impl Rock {
    fn from_cells(cells: Vec<Position>) -> Self {
        Self {
            cells: cells.clone(),
            width: cells.iter().map(|c| c.1).max().unwrap() + 1,
            height: cells.iter().map(|c| c.0).min().unwrap().abs() + 1,
        }
    }

    fn set_spawn(&mut self, highest: i32) {
        let x_offset = 2;
        let y_offset = highest + 3 + self.height;
        self.cells = self
            .cells
            .iter()
            .map(|c| (c.0 + y_offset, c.1 + x_offset))
            .collect();
    }

    fn move_to(
        &mut self,
        direction: Direction,
        occupied: &HashSet<Position>,
    ) -> Result<(), MoveError> {
        let to_add = match direction {
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Down => (-1, 0),
        };
        let cells: Vec<Position> = self
            .cells
            .iter()
            .map(|c| (c.0 + to_add.0, c.1 + to_add.1))
            .collect();
        if cells
            .iter()
            .any(|c| c.1 < 0 || c.1 > 6 || occupied.contains(c))
        {
            return Err(MoveError {});
        }
        self.cells = cells;
        Ok(())
    }

    fn get_max_height(&self) -> i32 {
        self.cells.iter().map(|c| c.0).max().unwrap()
    }
}

fn display(rocks: &HashSet<Position>, highest: i32, current: Option<&Rock>) {
    let mut lines = vec![];
    for _ in 0..highest + 10 {
        lines.push(vec!['|', '.', '.', '.', '.', '.', '.', '.', '|']);
    }
    for rock in rocks {
        lines[rock.0 as usize][rock.1 as usize + 1] = '#';
    }
    if let Some(r) = current {
        for cell in &r.cells {
            lines[cell.0 as usize][cell.1 as usize + 1] = '@';
        }
    }
    println!(
        "{}",
        lines
            .iter()
            .rev()
            .map(|l| l.iter().collect::<String>() + "\n")
            .collect::<String>()
    );
}

fn part_one(raw_data: &str) {
    let shift_sequence = get_shift_sequence(raw_data);
    let rock_sequence = vec![
        Rock::from_cells(vec![(0, 0), (0, 1), (0, 2), (0, 3)]),
        Rock::from_cells(vec![(0, 1), (-1, 0), (-1, 1), (-1, 2), (-2, 1)]),
        Rock::from_cells(vec![(0, 2), (-1, 2), (-2, 0), (-2, 1), (-2, 2)]),
        Rock::from_cells(vec![(0, 0), (-1, 0), (-2, 0), (-3, 0)]),
        Rock::from_cells(vec![(0, 0), (0, 1), (-1, 0), (-1, 1)]),
    ];
    let mut result = 0;
    let mut current_rock = 0;
    let mut current_shift = 0;
    let mut occupied_cells: HashSet<Position> = HashSet::new();
    occupied_cells.extend([(0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6)].iter());

    for _ in 0..2022 {
        let mut rock = rock_sequence[current_rock].clone();
        rock.set_spawn(result);
        //display(&occupied_cells, result, Some(&rock));
        loop {
            let shift = shift_sequence[current_shift].clone();
            current_shift = (current_shift + 1) % shift_sequence.len();
            let mut direction = match shift {
                Shift::Left => Direction::Left,
                Shift::Right => Direction::Right,
            };
            rock.move_to(direction, &occupied_cells);
            direction = Direction::Down;
            match rock.move_to(direction, &occupied_cells) {
                Ok(()) => (),
                Err(_) => break,
            }
            //display(&occupied_cells, result, Some(&rock));
        }

        occupied_cells.extend(rock.cells.iter());
        let max_in_rock = rock.get_max_height();
        if max_in_rock > result {
            result = max_in_rock;
        }

        current_rock = (current_rock + 1) % rock_sequence.len();
    }

    println!("Part 1 result is {result}")
}

fn part_two(raw_data: &str) {}
