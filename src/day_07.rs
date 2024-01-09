use std::{
    cell::RefCell, collections::HashMap, error::Error, fs::File, io::Read, rc::Rc, time::Instant,
};

use log::{debug, info};

use crate::ProblemPart;

pub fn solve(puzzle_input: &str, part: ProblemPart) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(puzzle_input)?;
    let mut puzzle_input = String::new();
    file.read_to_string(&mut puzzle_input)?;

    let result = match part {
        ProblemPart::One => {
            info!("Start solving part 1");
            let start = Instant::now();
            let result = solve_pt1(puzzle_input)?;
            let duration = start.elapsed().as_secs();
            info!("Solved part 1 in {duration} seconds.");
            result
        }
        ProblemPart::Two => {
            info!("Start solving part 2");
            let start = Instant::now();
            let result = solve_pt2(puzzle_input)?;
            let duration = start.elapsed().as_secs();
            info!("Solved part 2 in {duration} seconds.");
            result
        }
    };
    info!("Problem solution is {}", result);
    Ok(())
}

/// Filesystem enum has two variants:
/// - directory that has a name
/// - file that has a name and a size
enum NodeType {
    Directory(String),
    File(String, i32),
}

/// Node struct
///
/// It has an id that is registered in the arena, parent and children
/// are NodeId as well
struct Node {
    id: i32,
    parent: Option<i32>,
    children: Vec<i32>,
    depth: i32,
    node_type: NodeType,
}

struct TreeArena {
    map: HashMap<i32, Rc<RefCell<Node>>>,
    global_counter: i32,
    root: Option<i32>,
}

impl TreeArena {
    fn new() -> TreeArena {
        TreeArena {
            map: HashMap::new(),
            global_counter: 0,
            root: None,
        }
    }

    fn generate_id(&mut self) -> i32 {
        self.global_counter += 1;
        self.global_counter
    }

    fn get_node(&self, node_id: i32) -> Option<Rc<RefCell<Node>>> {
        self.map.get(&node_id).map(Rc::clone)
    }

    fn add_node(&mut self, parent: Option<i32>, node_type: NodeType) -> Result<i32, String> {
        let root_is_present = self.root.is_some();
        if parent.is_none() & root_is_present {
            Err("Root already exist".to_string())
        } else {
            let new_id = self.generate_id();

            let new_node = Node {
                id: new_id,
                parent,
                children: Vec::new(),
                depth: if let Some(parent_id) = parent {
                    self.get_node(parent_id).unwrap().borrow().depth + 1
                } else {
                    0
                },
                node_type,
            };

            self.map.insert(new_id, Rc::new(RefCell::new(new_node)));

            if let Some(parent_id) = parent {
                let parent_node = self.get_node(parent_id).unwrap();
                parent_node.borrow_mut().children.push(new_id);
            }

            if !root_is_present {
                self.root = Some(new_id);
            }

            Ok(new_id)
        }
    }

    fn get_root(&self) -> Option<Rc<RefCell<Node>>> {
        self.root.map(|node_id| self.get_node(node_id).unwrap())
    }

    fn print(&self, node_id: i32) {
        let ref_node = self.get_node(node_id).unwrap();
        let node = ref_node.borrow();
        let mut spaces = String::new();
        for _ in 0..node.depth {
            spaces.push(' ');
        }
        match &node.node_type {
            NodeType::Directory(name) => {
                println!("{spaces}- {name} (dir)");
                for child in node.children.iter() {
                    self.print(*child);
                }
            }
            NodeType::File(name, size) => {
                println!("{spaces}- ({name}, size={size})");
            }
        }
    }

    fn size(&self, node_id: i32) -> i32 {
        let ref_node = self.get_node(node_id).unwrap();
        let node = ref_node.borrow();
        let mut size = 0;
        match &node.node_type {
            NodeType::Directory(_) => {
                for child in node.children.iter() {
                    size += self.size(*child);
                }
            }
            NodeType::File(_, file_size) => {
                size += file_size;
            }
        }
        size
    }

    fn is_directory(&self, node_id: i32) -> bool {
        matches!(
            self.get_node(node_id).unwrap().borrow().node_type,
            NodeType::Directory(_)
        )
    }
}

fn parse_input(puzzle_input: String) -> TreeArena {
    let mut arena = TreeArena::new();

    // true if we are reading the ls output
    let mut ls_output = false;

    // we skip first because it is "cd /" so we add directly this root node
    let current_node_id = arena
        .add_node(None, NodeType::Directory("/".to_string()))
        .unwrap();
    let mut current_node = arena.get_node(current_node_id).unwrap();

    for line in puzzle_input.lines().skip(1) {
        if line.starts_with("$ ls") {
            ls_output = true;
        } else if let Some(stripped) = line.strip_prefix("$ cd ") {
            ls_output = false;
            if line.ends_with("..") {
                // we go to parent of current_node
                let parent = current_node.borrow().parent.unwrap();
                current_node = arena.get_node(parent).unwrap();
            } else {
                let directory_name = stripped.to_string();
                let parent_id = current_node.borrow().id;
                let current_node_id = arena
                    .add_node(Some(parent_id), NodeType::Directory(directory_name))
                    .unwrap();
                current_node = arena.get_node(current_node_id).unwrap();
            }
        } else if ls_output {
            if !line.starts_with("dir") {
                let file_size = line
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
                let file_name = line.split_whitespace().nth(1).unwrap().to_string();
                let parent_id = current_node.borrow().id;
                arena
                    .add_node(Some(parent_id), NodeType::File(file_name, file_size))
                    .unwrap();
            }
        } else {
            debug!("Unexpected branch");
        }
    }
    arena
}

fn solve_pt1(puzzle_input: String) -> Result<String, Box<dyn Error>> {
    let arena = parse_input(puzzle_input);
    arena.print(arena.get_root().unwrap().borrow().id);
    let size_th = 100000;
    let mut result = 0;
    for node_id in 1..=arena.global_counter {
        let size = arena.size(node_id);
        debug!("Size is {size}");
        if (size <= size_th) & arena.is_directory(node_id) {
            result += size;
        }
    }
    Ok(result.to_string())
}

fn solve_pt2(puzzle_input: String) -> Result<String, Box<dyn Error>> {
    let arena = parse_input(puzzle_input);
    arena.print(arena.get_root().unwrap().borrow().id);
    let required_space = 30000000;
    let total_disk_space = 70000000;
    let available_space = total_disk_space - arena.size(arena.get_root().unwrap().borrow().id);
    let space_to_free = required_space - available_space;
    let mut candidates_to_delete: Vec<i32> = Vec::new();
    for node_id in 1..=arena.global_counter {
        let size = arena.size(node_id);
        if (size >= space_to_free) & arena.is_directory(node_id) {
            candidates_to_delete.push(size)
        }
    }
    println!("candiates_to_delete \n{:?}", candidates_to_delete);
    Ok(candidates_to_delete.iter().min().unwrap().to_string())
}

#[cfg(test)]
mod test {
    use std::{error::Error, fs::File, io::Read};

    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_pt1() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/day_07_example.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        let result = solve_pt1(puzzle_input)?;

        assert_eq!("95437".to_string(), result);

        Ok(())
    }

    #[test]
    fn test_pt2() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/day_07_example.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        let result = solve_pt2(puzzle_input)?;

        assert_eq!("24933642".to_string(), result);

        Ok(())
    }
}
