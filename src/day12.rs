use std::collections::HashMap;

pub fn day12(input_lines: &[String]) -> (u64, u64) {
    let caves = parse_input(input_lines);
    let part1 = explore(&caves, false);
    let part2 = explore(&caves, true);
    (part1,part2)
}

fn parse_input(input_lines: &[String]) -> Vec<Cave> {
    let mut cave_dict: HashMap<&str, usize> = HashMap::with_capacity(input_lines.len());
    let mut caves: Vec<Cave> = Vec::with_capacity(input_lines.len());

    caves.push(Cave::new("start", 0));
    cave_dict.insert("start", 0);
    let mut cave_id = 1usize;

    for line in input_lines {
        let connected_caves = line.split('-').collect::<Vec<_>>();
        let cave1_id = *cave_dict.entry(connected_caves[0]).or_insert(cave_id);
        if cave1_id == cave_id {
            let cave = Cave::new(connected_caves[0], cave_id);
            caves.push(cave);
            cave_id += 1;
        }
        let cave2_id = *cave_dict.entry(connected_caves[1]).or_insert(cave_id);
        if cave2_id == cave_id {
            let cave = Cave::new(connected_caves[1], cave_id);
            caves.push(cave);
            cave_id += 1;
        }
        if cave2_id != 0 {
            caves[cave1_id].connections.push(cave2_id);
        }
        if cave1_id != 0 {
            caves[cave2_id].connections.push(cave1_id);
        }
    }

    caves
}

fn explore(caves: &[Cave], allow_duplicate_small_cave: bool) -> u64 {
    let mut num_paths = 0u64;
    // Cave 0 is always the start
    let mut paths_in_progress: Vec<Path> = vec![Path::new(&caves[0])];

    while let Some(path) = paths_in_progress.pop() {
        let cave = &caves[path.current_position];
        if cave.end {
            num_paths += 1;
        } else {
            for connection in cave.connections.iter() {
                if let Some(new_path) = Path::add(&path, &caves[*connection], allow_duplicate_small_cave) {
                    paths_in_progress.push(new_path);
                }
            }
        }
    }

    num_paths
}

struct Cave {
    id: usize,
    small: bool,
    end: bool,
    connections: Vec<usize>,
}

impl Cave {
    fn new(name: &str, id: usize) -> Self {
        let small = name.chars().next().unwrap().is_lowercase();
        let end = name == "end";
        Self {
            id,
            small,
            end,
            connections: Vec::new(),
        }
    }
}

#[derive(Clone)]
struct Path {
    current_position: usize,
    small_caves_visited: usize,
    duplicate_small_cave: bool,
}

impl Path {
    fn new(first_cave: &Cave) -> Self {
        Self {
            current_position: first_cave.id,
            small_caves_visited: 0,
            duplicate_small_cave: false,
        }
        // We assume that the first cave is "start", which means we don't need to do any other
        // processing like marking that we've visited a small cave (since you can't return to
        // "start" anyway).
    }

    fn add(previous: &Self, next_step: &Cave, allow_duplicate_small_cave: bool) -> Option<Self> {
        assert!(next_step.id != 0);
        let cave_is_small = next_step.small;
        let visited_small_cave_before = cave_is_small && (previous.small_caves_visited & (1 << next_step.id) != 0);
        if visited_small_cave_before && (!allow_duplicate_small_cave || previous.duplicate_small_cave) {
            // This path has already visited this small cave, and either we're not
            // allowed to visit one twice, or we've already visited one twice.
            None
        } else {
            let mut new_path = previous.clone();
            new_path.current_position = next_step.id;
            if cave_is_small {
                if visited_small_cave_before {
                    new_path.duplicate_small_cave = true;
                } else {
                    new_path.small_caves_visited |= 1 << next_step.id;
                }
            }
            Some(new_path)
        }
    }
}