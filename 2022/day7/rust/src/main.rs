use std::{cell::RefCell, collections::HashMap, error::Error, fmt::Display, fs, rc::Rc};

fn main() {
    let data = get_input().unwrap();
    part_one(&data);
    part_two(&data);
}

fn get_input() -> Result<String, Box<dyn Error>> {
    let data = fs::read_to_string("./data/data.txt")?;
    Ok(data)
}

#[derive(Debug, Clone)]
struct File {
    _name: String,
    size: usize,
}

type Link<T> = Rc<RefCell<T>>;

#[derive(Debug, Clone)]
struct Dir {
    name: String,
    files: Vec<File>,
    parent: Option<Link<Dir>>,
    directories: HashMap<String, Link<Dir>>,
}

impl Dir {
    fn new(name: &str, parent: Option<Link<Dir>>) -> Self {
        Self {
            name: name.to_string(),
            files: vec![],
            parent,
            directories: HashMap::new(),
        }
    }
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parent_name = if let Some(parent) = &self.parent {
            parent.as_ref().borrow().name.clone()
        } else {
            "None".to_string()
        };
        write!(
            f,
            "name: {}\nparent: {}\nfiles: {:?}\n",
            self.name, parent_name, self.files
        )?;
        for (_key, value) in &self.directories {
            write!(f, "dir: {}", value.as_ref().borrow())?;
        }
        Ok(())
    }
}

fn parse_input(raw_data: &str) -> Link<Dir> {
    let lines: Vec<&str> = raw_data.split("\n").collect();
    let origin = Rc::new(RefCell::new(Dir::new("/", None)));
    let mut current_dir = origin.clone();
    let mut lines = lines.iter();
    let mut parent = None;
    lines.next();
    while let Some(line) = lines.next() {
        if line.starts_with("$ ls") {
            continue;
        } else if line.starts_with("$ cd") {
            let to = &line[5..];
            if to.contains("..") {
                current_dir = parent.unwrap();
                parent = current_dir.as_ref().borrow().parent.clone();
            } else {
                parent = Some(current_dir.clone());
                current_dir = current_dir
                    .clone()
                    .as_ref()
                    .borrow()
                    .directories
                    .get(to)
                    .unwrap()
                    .clone();
            }
        } else if line.starts_with("dir") {
            let dir = Rc::new(RefCell::new(Dir::new(
                &line[4..],
                Some(current_dir.clone()),
            )));
            current_dir
                .borrow_mut()
                .directories
                .insert(line[4..].to_string(), dir);
        } else {
            let data = line.split(' ').collect::<Vec<&str>>();
            let size = data[0].parse::<usize>().unwrap();
            let name = data[1];
            current_dir.borrow_mut().files.push(File {
                _name: name.to_string(),
                size,
            });
        }
    }
    origin
}

fn get_total_size(result: &mut usize, dir: Link<Dir>) -> usize {
    let mut size = 0;
    size += dir
        .as_ref()
        .borrow()
        .files
        .iter()
        .map(|f| f.size)
        .sum::<usize>();
    for (_key, value) in &dir.as_ref().borrow().directories {
        let sub_size = get_total_size(result, value.clone());
        if sub_size < 100000 {
            *result += sub_size;
        }
        size += sub_size;
    }
    size
}

fn part_one(raw_data: &str) {
    let dir_tree = parse_input(raw_data);
    let mut result = 0;
    get_total_size(&mut result, dir_tree);
    println!("Part 1 result is {result}");
}

fn part_two(raw_data: &str) {}
