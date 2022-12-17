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

#[derive(Debug)]
struct Room {
    name: String,
    adjacent_rooms: Vec<String>,
    flow_rate: u32,
}

fn get_rooms(raw_data: &str) -> Vec<Room> {
    let lines: Vec<&str> = raw_data.split("\n").collect();
    let mut room_map = vec![];
    for line in lines {
        let (valve, leading_to) = line.split_once("; ").unwrap();
        let (name, flow_rate) = valve[6..].split_once(" has flow rate=").unwrap();
        let flow_rate = flow_rate.parse().unwrap();
        let intro_index = if leading_to.chars().nth(6).unwrap() == 's' {
            23
        } else {
            22
        };
        let adjacent_rooms: Vec<String> = leading_to[intro_index..]
            .split(", ")
            .map(|s| s.to_string())
            .collect();

        let room = Room {
            name: name.to_string(),
            adjacent_rooms: adjacent_rooms.clone(),
            flow_rate,
        };
        room_map.push(room);
        //println!("name: {name}, flow: {flow_rate}, adj: {adjacent_rooms:?}");
    }
    room_map
}

struct RoomNode {
    name: String,
    flow_rate: u32,
    cost_map: HashMap<String, u32>,
}

fn reduce_map(rooms: Vec<Room>) -> Vec<RoomNode> {
    let mut result = vec![];
    for room in rooms.iter() {
        if room.flow_rate == 0 && room.name != "AA" {
            continue;
        }
        let mut cost_map = HashMap::new();
        let mut visited = HashSet::new();
        let mut to_visit = VecDeque::new();
        to_visit.push_back((room.name.clone(), 0));
        while !to_visit.is_empty() {
            let current = to_visit.pop_front().unwrap();
            let current_room = rooms.iter().find(|r| r.name == current.0).unwrap();
            if visited.contains(&current.0) {
                continue;
            }
            if current_room.flow_rate > 0 {
                cost_map.insert(current.0.to_string(), current.1 + 1);
            }
            visited.insert(current.0);
            for room in &current_room.adjacent_rooms {
                to_visit.push_back((room.to_string(), current.1 + 1));
            }
        }
        result.push(RoomNode {
            name: room.name.clone(),
            flow_rate: room.flow_rate,
            cost_map,
        })
    }
    result
}

fn find<'a>(name: &str, rooms: &'a [RoomNode]) -> &'a RoomNode {
    rooms.iter().find(|r| r.name == name).unwrap()
}

fn part_one(raw_data: &str) {
    let rooms_graph = reduce_map(get_rooms(raw_data));
    let time_limit = 30;
    let mut to_visit = vec![(find("AA", &rooms_graph), 0, 0, vec![])];
    let mut result = 0;
    while !to_visit.is_empty() {
        let (room, pressure, cost, visited) = to_visit.pop().unwrap();
        let pressure = pressure + (time_limit - cost) * room.flow_rate;
        if pressure > result {
            result = pressure;
        }
        let mut new_visited = visited;
        new_visited.push(room.name.clone());
        for (r, c) in &room.cost_map {
            if !new_visited.contains(r) {
                if cost + c < time_limit {
                    to_visit.push((
                        find(&r, &rooms_graph),
                        pressure,
                        cost + c,
                        new_visited.clone(),
                    ));
                }
            }
        }
    }
    println!("Part 1 result is {result}")
}

fn part_two(raw_data: &str) {}
