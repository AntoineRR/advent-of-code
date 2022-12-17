use std::{error::Error, fmt::Display, fs};

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

#[derive(Debug)]
struct MoveError {}

impl Display for MoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot move rock")
    }
}

#[derive(Clone)]
struct Rock {
    cells: Vec<u8>,
    offset: u64,
}

impl Rock {
    fn from_cells(cells: Vec<u8>) -> Self {
        Self { cells, offset: 0 }
    }

    fn set_spawn(&mut self, highest: u64) {
        let x_offset = 2;
        let y_offset = highest + 3 + self.cells.len() as u64;
        self.cells = self.cells.iter().map(|c| c >> x_offset).collect();
        self.offset = y_offset;
    }

    fn move_to(&mut self, direction: Direction, occupied: &OccupiedLines) -> Result<(), MoveError> {
        let mut cells = self.cells.clone();
        let mut offset = self.offset;
        match direction {
            Direction::Left => {
                if cells.iter().any(|c| c & 0b10000000 > 1) {
                    return Err(MoveError {});
                }
                cells = cells.iter().map(|c| c << 1).collect()
            }
            Direction::Right => {
                if cells.iter().any(|c| c & 0b00000010 > 1) {
                    return Err(MoveError {});
                }
                cells = cells.iter().map(|c| c >> 1).collect()
            }
            Direction::Down => offset -= 1,
        };

        for (i, cell) in cells.iter().rev().enumerate() {
            let current_offset = offset - (cells.len() - 1 - i) as u64;
            if ((occupied.offset - (occupied.occupied.len() - 1) as u64)..(occupied.offset + 1))
                .contains(&current_offset)
            {
                if occupied.occupied
                    [occupied.occupied.len() - 1 - (occupied.offset - current_offset) as usize]
                    & cell
                    > 0
                {
                    return Err(MoveError {});
                }
            }
        }

        self.cells = cells;
        self.offset = offset;
        Ok(())
    }

    fn get_max_height(&self) -> u64 {
        self.offset
    }
}

struct OccupiedLines {
    occupied: Vec<u8>,
    offset: u64,
}

impl OccupiedLines {
    fn add_rock(&mut self, rock: &Rock) {
        for (i, cell) in rock.cells.iter().rev().enumerate() {
            let current_offset = rock.offset - (rock.cells.len() - 1 - i) as u64;
            if ((self.offset - (self.occupied.len() - 1) as u64)..(self.offset + 1))
                .contains(&current_offset)
            {
                let current_len = self.occupied.len();
                self.occupied[current_len - 1 - (self.offset - current_offset) as usize] |= cell;
            } else {
                self.occupied.push(*cell);
                self.offset += 1;
                if self.occupied.len() > 100 {
                    self.occupied.remove(0);
                }
            }
        }
    }
}

fn display(rocks: &OccupiedLines, highest: u64, current: Option<&Rock>) {
    let mut lines = vec![];
    for _ in 0..highest + 10 {
        lines.push(vec!['|', '.', '.', '.', '.', '.', '.', '.', '|']);
    }
    for (i, rock) in rocks.occupied.iter().rev().enumerate() {
        for j in 0..8 {
            if (rock << j) & 0b10000000 > 1 {
                lines[rocks.offset as usize - i][j + 1] = '#';
            }
        }
    }
    if let Some(r) = current {
        for (i, cell) in r.cells.iter().enumerate() {
            for j in 0..8 {
                if (cell << j) & 0b10000000 > 1 {
                    lines[r.offset as usize - i][j + 1] = '@';
                }
            }
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
        Rock::from_cells(vec![0b11110000]),
        Rock::from_cells(vec![0b01000000, 0b11100000, 0b01000000]),
        Rock::from_cells(vec![0b00100000, 0b00100000, 0b11100000]),
        Rock::from_cells(vec![0b10000000, 0b10000000, 0b10000000, 0b10000000]),
        Rock::from_cells(vec![0b11000000, 0b11000000]),
    ];
    let mut result = 0;
    let mut current_rock = 0;
    let mut current_shift = 0;
    let mut occupied_cells = OccupiedLines {
        occupied: vec![0b11111110],
        offset: 0,
    };

    for _ in 0..1_000_000 {
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

        occupied_cells.add_rock(&rock);
        let max_in_rock = rock.get_max_height();
        if max_in_rock > result {
            result = max_in_rock;
        }

        current_rock = (current_rock + 1) % rock_sequence.len();
    }

    println!("Part 1 result is {result}")
}

fn part_two(raw_data: &str) {}
