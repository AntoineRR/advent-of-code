use std::{
    collections::HashMap,
    error::Error,
    fs,
    iter::Sum,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
};

fn main() {
    let data = get_input().unwrap();
    part_one(&data);
    part_two(&data);
}

fn get_input() -> Result<String, Box<dyn Error>> {
    let data = fs::read_to_string("./data/example.txt")?;
    Ok(data)
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct Resources {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Sub for Resources {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
    }
}

impl SubAssign for Resources {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode,
        };
    }
}

impl Add for Resources {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl AddAssign for Resources {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
            geode: self.geode + other.geode,
        };
    }
}

impl Mul<usize> for Resources {
    type Output = Resources;

    fn mul(self, rhs: usize) -> Self::Output {
        Self {
            ore: self.ore * rhs,
            clay: self.clay * rhs,
            obsidian: self.obsidian * rhs,
            geode: self.geode * rhs,
        }
    }
}

impl Sum for Resources {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Resources::default(), |acc, r| acc + r)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Robot {
    cost: Resources,
    gather: Resources,
}

impl Default for Robot {
    fn default() -> Self {
        Self {
            cost: Resources::default(),
            gather: Resources {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    index: usize,
    robots: Vec<Robot>,
}

fn parse_input(raw_data: &str) -> Vec<Blueprint> {
    raw_data
        .split("\n")
        .map(|l| {
            let (raw_index, plans) = l.split_once(": ").unwrap();
            let index = raw_index.split_once(' ').unwrap().1.parse().unwrap();
            let raw_robots: Vec<&str> = plans.split(". ").collect();
            let robots = vec![
                Robot {
                    cost: Resources {
                        ore: raw_robots[0][21..]
                            .split(' ')
                            .next()
                            .unwrap()
                            .parse()
                            .unwrap(),
                        clay: 0,
                        obsidian: 0,
                        geode: 0,
                    },
                    gather: Resources {
                        ore: 1,
                        clay: 0,
                        obsidian: 0,
                        geode: 0,
                    },
                },
                Robot {
                    cost: Resources {
                        ore: raw_robots[1][22..]
                            .split(' ')
                            .next()
                            .unwrap()
                            .parse()
                            .unwrap(),
                        clay: 0,
                        obsidian: 0,
                        geode: 0,
                    },
                    gather: Resources {
                        ore: 0,
                        clay: 1,
                        obsidian: 0,
                        geode: 0,
                    },
                },
                Robot {
                    cost: Resources {
                        ore: raw_robots[2][26..]
                            .split(' ')
                            .next()
                            .unwrap()
                            .parse()
                            .unwrap(),
                        clay: raw_robots[2][36..]
                            .split(' ')
                            .next()
                            .unwrap()
                            .parse()
                            .unwrap(),
                        obsidian: 0,
                        geode: 0,
                    },
                    gather: Resources {
                        ore: 0,
                        clay: 0,
                        obsidian: 1,
                        geode: 0,
                    },
                },
                Robot {
                    cost: Resources {
                        ore: raw_robots[3][23..]
                            .split(' ')
                            .next()
                            .unwrap()
                            .parse()
                            .unwrap(),
                        clay: 0,
                        obsidian: raw_robots[3][33..]
                            .split(' ')
                            .next()
                            .unwrap()
                            .parse()
                            .unwrap(),
                        geode: 0,
                    },
                    gather: Resources {
                        ore: 0,
                        clay: 0,
                        obsidian: 0,
                        geode: 1,
                    },
                },
            ];
            Blueprint { index, robots }
        })
        .collect()
}

fn available_robots(blueprint: &Blueprint, resources: &Resources) -> Vec<Robot> {
    blueprint
        .robots
        .iter()
        .filter(|r| {
            r.cost.ore <= resources.ore
                && r.cost.clay <= resources.clay
                && r.cost.obsidian <= resources.obsidian
                && r.cost.geode <= resources.geode
        })
        .rev() // rev to get the geode robots first
        .cloned()
        .collect()
}

fn harvest(robots: &[(Robot, usize)]) -> Resources {
    robots.iter().map(|(r, n)| r.gather.clone() * *n).sum()
}

fn get_best(
    blueprint: &Blueprint,
    robots: Vec<(Robot, usize)>,
    mut resources: Resources,
    memoization: &mut HashMap<(Vec<(Robot, usize)>, Resources), usize>,
    time_limit: usize,
) -> usize {
    if time_limit <= 0 {
        return resources.geode;
    }
    if let Some(r) = memoization.get(&(robots.clone(), resources.clone())) {
        return *r;
    }
    resources += harvest(&robots);
    let mut max = if resources.ore < 2 {
        get_best(
            blueprint,
            robots.clone(),
            resources.clone(),
            memoization,
            time_limit - 1,
        )
    } else {
        resources.geode
    };
    for robot in available_robots(&blueprint, &resources) {
        let new_resources = resources.clone() - robot.cost.clone();
        let mut new_robots = robots.clone();
        let (i, _) = new_robots
            .iter()
            .enumerate()
            .find(|(_, (r, _))| *r == robot)
            .unwrap();
        new_robots[i].1 += 1;
        let result = get_best(
            blueprint,
            new_robots,
            new_resources,
            memoization,
            time_limit - 1,
        );
        if result > max {
            max = result;
        }
    }
    memoization.insert((robots, resources), max);
    max
}

fn part_one(raw_data: &str) {
    let blueprints = parse_input(raw_data);
    let time_limit = 24;
    let mut result = 0;
    for blueprint in blueprints {
        let robots = vec![Robot::default()];
        let resources = Resources::default();
        let max = get_best(
            &blueprint,
            robots,
            resources,
            &mut HashMap::new(),
            time_limit,
        );
        result += max * blueprint.index;
    }
    println!("Part 1 result is {result}")
}

fn part_two(raw_data: &str) {}
