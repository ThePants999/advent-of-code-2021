use std::{collections::{HashSet, HashMap}, hash::Hash};

pub fn day12(input_lines: &[String]) -> (u64, u64) {
    let (caves, cave_dict) = parse_input(input_lines);
    let start = *cave_dict.get("start").unwrap();
    let part1 = explore(&caves, start, false).len() as u64;
    let part2 = explore(&caves, start, true).len() as u64;
    (part1,part2)
}

fn parse_input<'a>(input_lines: &'a [String]) -> (Vec<Cave<'a>>, HashMap<&'a str, usize>) {
    let mut cave_id = 0usize;
    let mut cave_dict: HashMap<&'a str, usize> = HashMap::with_capacity(input_lines.len());
    let mut caves: Vec<Cave> = Vec::with_capacity(input_lines.len());
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
        if connected_caves[1] != "start" {
            caves[cave1_id].connections.push(cave2_id);
        }
        if connected_caves[0] != "start" {
            caves[cave2_id].connections.push(cave1_id);
        }
    }
    (caves, cave_dict)
}

fn explore(caves: &[Cave], start_cave_index: usize, allow_duplicate_small_cave: bool) -> Vec<Path> {
    let mut paths: Vec<Path> = Vec::new();
    let mut paths_in_progress: Vec<Path> = Vec::new();
    let mut paths_tried: HashSet<Path> = HashSet::new();

    paths_in_progress.push(Path::new(&caves[start_cave_index]));
    while let Some(path) = paths_in_progress.pop() {
        let cave = &caves[*path.path.last().unwrap()];
        if cave.name == "end" {
            paths.push(path);
        } else {
            for connection in cave.connections.iter() {
                if let Some(new_path) = Path::add(&path, &caves[*connection], allow_duplicate_small_cave) {
                    if !paths_tried.contains(&new_path) {
                        paths_tried.insert(new_path.clone());
                        paths_in_progress.push(new_path);
                    }
                }
            }
        }
    }
    paths
}

struct Cave<'a> {
    name: &'a str,
    id: usize,
    small: bool,
    connections: Vec<usize>,
}

impl<'a> Cave<'a> {
    fn new(name: &'a str, id: usize) -> Self {
        let small = name.chars().next().unwrap().is_lowercase();
        Self {
            name,
            id,
            small,
            connections: Vec::new(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Path {
    path: Vec<usize>,
    small_caves: usize,
    duplicate_small_cave: bool,
}

impl Path {
    fn new(first_cave: &Cave) -> Self {
        let path = Self {
            path: Vec::new(),
            small_caves: 0,
            duplicate_small_cave: false,
        };
        Self::add(&path, first_cave, false).unwrap()
    }

    fn add(previous: &Self, next_step: &Cave, allow_duplicate_small_cave: bool) -> Option<Self> {
        let cave_is_small = next_step.small;
        let visited_small_cave_before = cave_is_small && (previous.small_caves & (1 << next_step.id) != 0);
        if visited_small_cave_before && (!allow_duplicate_small_cave || previous.duplicate_small_cave) {
            // This path has already visited this small cave, and either we're not
            // allowed to visit one twice, or we've already visited one twice.
            None
        } else {
            let mut new_path = previous.clone();
            new_path.path.push(next_step.id);
            if cave_is_small {
                if visited_small_cave_before {
                    new_path.duplicate_small_cave = true;
                } else {
                    new_path.small_caves |= 1 << next_step.id;
                }
            }
            Some(new_path)
        }
    }
}