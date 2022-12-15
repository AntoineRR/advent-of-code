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

type Position = (i32, i32);

struct SourceWithBeacon {
    source: Position,
    closest_beacon: Position,
}

impl SourceWithBeacon {
    fn from_line(line: &str) -> Self {
        let (source, beacon) = line
            .split_once(":")
            .map(|(raw_source, raw_beacon)| {
                let source = raw_source[12..]
                    .split_once(", y=")
                    .map(|p| (p.0.parse().unwrap(), p.1.parse().unwrap()))
                    .unwrap();
                let beacon = raw_beacon[24..]
                    .split_once(", y=")
                    .map(|p| (p.0.parse().unwrap(), p.1.parse().unwrap()))
                    .unwrap();
                (source, beacon)
            })
            .unwrap();
        Self {
            source,
            closest_beacon: beacon,
        }
    }
}

fn get_manhattan_distance(a: Position, b: Position) -> u32 {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn part_one(raw_data: &str) {
    let lines: Vec<&str> = raw_data.split("\n").collect();
    let sources: Vec<SourceWithBeacon> = lines
        .iter()
        .map(|l| SourceWithBeacon::from_line(l))
        .collect();
    let line_y = 2_000_000;
    let mut scanned = HashSet::new();
    let beacons_on_line: HashSet<Position> = sources
        .iter()
        .filter(|s| s.closest_beacon.1 == line_y)
        .map(|s| s.closest_beacon)
        .collect();
    for source in &sources {
        let distance_from_beacon = get_manhattan_distance(source.source, source.closest_beacon);
        let distance_from_line = get_manhattan_distance(source.source, (source.source.0, line_y));
        if distance_from_beacon >= distance_from_line {
            let diff: i32 = (distance_from_beacon - distance_from_line)
                .try_into()
                .unwrap();
            let start = source.source.0 - diff;
            let end = source.source.0 + diff;
            for i in start..=end {
                scanned.insert(i);
            }
        }
    }
    let result = scanned.len() - beacons_on_line.len();
    println!("Part 1 result is {result}")
}

fn part_two(raw_data: &str) {}
