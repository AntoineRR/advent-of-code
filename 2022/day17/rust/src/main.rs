// Part two doesn't work for now!

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
}

fn is_line_useless(upper: &[u8]) -> bool {
    upper.iter().fold(0, |acc, l| acc | l) == 0b11111110
}

#[derive(Clone)]
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
            }
        }
        while is_line_useless(&self.occupied[1..]) {
            self.occupied.remove(0);
        }
    }
}

fn display(rocks: &OccupiedLines, highest: u64, current: Option<&Rock>) {
    let mut lines = vec![];
    let low = highest - rocks.occupied.len() as u64;
    for _ in low..highest + 10 {
        lines.push(vec!['|', '.', '.', '.', '.', '.', '.', '.', '|']);
    }
    for (i, rock) in rocks.occupied.iter().rev().enumerate() {
        for j in 0..8 {
            if (rock << j) & 0b10000000 > 1 {
                lines[rocks.offset as usize - i - low as usize][j + 1] = '#';
            }
        }
    }
    if let Some(r) = current {
        for (i, cell) in r.cells.iter().enumerate() {
            for j in 0..8 {
                if (cell << j) & 0b10000000 > 1 {
                    lines[r.offset as usize - i - low as usize][j + 1] = '@';
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

fn get_height(
    rock_sequence: &[Rock],
    shift_sequence: &[Shift],
    n_rocks: usize,
    current_rock: &mut usize,
    current_shift: &mut usize,
    occupied_cells: &mut OccupiedLines,
    patterns_to_match: Option<HashSet<(Vec<u8>, usize, usize)>>,
    encountered: &mut Option<HashSet<Vec<u8>>>,
) -> Option<usize> {
    for i in 0..n_rocks {
        let mut rock = rock_sequence[*current_rock].clone();
        rock.set_spawn(occupied_cells.offset);
        //display(&occupied_cells, result, Some(&rock));
        loop {
            let shift = shift_sequence[*current_shift].clone();
            *current_shift = (*current_shift + 1) % shift_sequence.len();
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
        if let Some(e) = encountered {
            e.insert(occupied_cells.occupied.clone());
        }
        if let Some(pattern) = &patterns_to_match {
            if pattern.contains(&(
                occupied_cells.occupied.clone(),
                *current_rock,
                *current_shift,
            )) {
                println!("found it!");
                return Some(i);
            }
        }
        *current_rock = (*current_rock + 1) % rock_sequence.len();
    }
    None
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
    let mut current_rock = 0;
    let mut current_shift = 0;
    let mut occupied_cells = OccupiedLines {
        occupied: vec![0b11111110],
        offset: 0,
    };
    get_height(
        &rock_sequence,
        &shift_sequence,
        2022,
        &mut current_rock,
        &mut current_shift,
        &mut occupied_cells,
        None,
        &mut None,
    );
    let result = occupied_cells.offset;

    println!("Part 1 result is {result}")
}

fn part_two(raw_data: &str) {
    let shift_sequence = get_shift_sequence(raw_data);
    let rock_sequence = vec![
        Rock::from_cells(vec![0b11110000]),
        Rock::from_cells(vec![0b01000000, 0b11100000, 0b01000000]),
        Rock::from_cells(vec![0b00100000, 0b00100000, 0b11100000]),
        Rock::from_cells(vec![0b10000000, 0b10000000, 0b10000000, 0b10000000]),
        Rock::from_cells(vec![0b11000000, 0b11000000]),
    ];
    let mut current_rock = 0;
    let mut current_shift = 0;
    let mut occupied_cells = OccupiedLines {
        occupied: vec![0b11111110],
        offset: 0,
    };
    let encountered = HashSet::new();
    get_height(
        &rock_sequence,
        &shift_sequence,
        1_000_000,
        &mut current_rock,
        &mut current_shift,
        &mut occupied_cells,
        None,
        &mut Some(encountered.clone()),
    );

    display(&occupied_cells, occupied_cells.offset, None);
    let pattern_to_match = Some(
        encountered
            .iter()
            .map(|e| (e.clone(), current_rock, current_shift))
            .collect::<HashSet<(Vec<u8>, usize, usize)>>(),
    );
    let n = get_height(
        &rock_sequence,
        &shift_sequence,
        1_000_000_000,
        &mut current_rock,
        &mut current_shift,
        &mut occupied_cells,
        pattern_to_match,
        &mut None,
    )
    .unwrap();
    let height = occupied_cells.offset;
    let mut result = height * (1_000_000_000_000 / (n - 1_000_000)) as u64;
    let rest = 1_000_000_000_000 / (n - 1_000_000);
    get_height(
        &rock_sequence,
        &shift_sequence,
        rest,
        &mut current_rock,
        &mut current_shift,
        &mut occupied_cells,
        None,
        &mut None,
    );
    result += occupied_cells.offset - height;

    println!("Part 2 result is {result}")
}
