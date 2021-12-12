use std::{collections::{HashSet, HashMap}, rc::{Rc, Weak}, cell::RefCell, hash::{Hash, Hasher}};

pub fn day12(input_lines: &[String]) -> (u64, u64) {
    let caves = parse_input(input_lines);
    let start = caves.get("start").unwrap();
    let part1 = explore(start, false).len() as u64;
    let part2 = explore(start, true).len() as u64;
    (part1,part2)
}

fn parse_input<'a>(input_lines: &'a [String]) -> HashMap<&'a str, Rc<RefCell<Cave>>> {
    let mut caves: HashMap<&'a str, Rc<RefCell<Cave>>> = HashMap::with_capacity(input_lines.len());
    for line in input_lines {
        let connected_caves = line.split('-').collect::<Vec<_>>();
        let cave1 = Rc::downgrade(caves.entry(connected_caves[0]).or_insert_with(|| Rc::new(RefCell::new(Cave::new(connected_caves[0])))));
        let cave2 = caves.entry(connected_caves[1]).or_insert_with(|| Rc::new(RefCell::new(Cave::new(connected_caves[1]))));
        if connected_caves[1] != "start" {
            cave1.upgrade().unwrap().borrow_mut().connections.push(Rc::downgrade(cave2));
        }
        if connected_caves[0] != "start" {
            cave2.borrow_mut().connections.push(cave1);
        }
    }
    caves
}

fn explore<'a>(start_cave: &Rc<RefCell<Cave<'a>>>, allow_duplicate_small_cave: bool) -> Vec<Path<'a>> {
    let mut paths: Vec<Path<'a>> = Vec::new();
    let mut paths_in_progress: Vec<Path<'a>> = Vec::new();
    let mut paths_tried: HashSet<Path<'a>> = HashSet::new();

    paths_in_progress.push(Path::new(&Rc::downgrade(start_cave)));
    while let Some(path) = paths_in_progress.pop() {
        let cave_rc = path.path.last().unwrap().get_cave();
        let cave = cave_rc.borrow();
        if cave.id == "end" {
            paths.push(path);
        } else {
            for connection in cave.connections.iter() {
                if let Some(new_path) = Path::add(&path, connection, allow_duplicate_small_cave) {
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
    id: &'a str,
    small: bool,
    connections: Vec<Weak<RefCell<Cave<'a>>>>,
}

impl<'a> Cave<'a> {
    fn new(id: &'a str) -> Self {
        let small = id.chars().next().unwrap().is_lowercase();
        Self {
            id,
            small,
            connections: Vec::new(),
        }
    }
}

impl<'a> PartialEq for Cave<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<'a> Eq for Cave<'a> {}

impl<'a> Hash for Cave<'a> {
    fn hash<H>(&self, state: &mut H)
        where H: Hasher
    {
        self.id.hash(state);
    }
}

struct CaveRef<'a>(Weak<RefCell<Cave<'a>>>);

impl<'a> CaveRef<'a> {
    fn from(cave: &Weak<RefCell<Cave<'a>>>) -> Self {
        Self {
            0: Weak::clone(cave)
        }
    }

    fn get_cave(&self) -> Rc<RefCell<Cave<'a>>> {
        self.0.upgrade().unwrap()
    }
}

impl<'a> PartialEq for CaveRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0.ptr_eq(&other.0)
    }
}

impl<'a> Eq for CaveRef<'a> {}

impl<'a> Clone for CaveRef<'a> {
    fn clone(&self) -> Self {
        Self {
            0: Weak::clone(&self.0)
        }
    }
}

impl<'a> Hash for CaveRef<'a> {
    fn hash<H>(&self, state: &mut H)
        where H: Hasher
    {
        self.0.upgrade().unwrap().borrow().id.hash(state);
    }
}

#[derive(Clone)]
struct Path<'a> {
    path: Vec<CaveRef<'a>>,
    small_caves: HashSet<CaveRef<'a>>,
    duplicate_small_cave: bool,
}

impl<'a> Path<'a> {
    fn new(first_cave: &Weak<RefCell<Cave<'a>>>) -> Self {
        let path = Self {
            path: Vec::new(),
            small_caves: HashSet::new(),
            duplicate_small_cave: false,
        };
        Self::add(&path, first_cave, false).unwrap()
    }

    fn add(previous: &Self, next_step: &Weak<RefCell<Cave<'a>>>, allow_duplicate_small_cave: bool) -> Option<Self> {
        let caveref = CaveRef::from(next_step);
        let cave_is_small = caveref.get_cave().borrow().small;
        let visited_small_cave_before = cave_is_small && previous.small_caves.contains(&caveref);
        if visited_small_cave_before && (!allow_duplicate_small_cave || previous.duplicate_small_cave) {
            // This path has already visited this small cave, and either we're not
            // allowed to visit one twice, or we've already visited one twice.
            None
        } else {
            let mut new_path = previous.clone();
            new_path.path.push(caveref.clone());
            if cave_is_small {
                if visited_small_cave_before {
                    new_path.duplicate_small_cave = true;
                } else {
                    new_path.small_caves.insert(caveref);
                }
            }
            Some(new_path)
        }
    }
}

impl<'a> PartialEq for Path<'a> {
    fn eq(&self, other: &Self) -> bool {
        let mut eq = true;
        if self.path.len() == other.path.len() {
            for index in 0..self.path.len() {
                if !self.path[index].0.ptr_eq(&other.path[index].0) {
                    eq = false;
                    break;
                }
            }
        } else {
            eq = false;
        }
        eq
    }
}

impl<'a> Eq for Path<'a> {}

impl<'a> Hash for Path<'a> {
    fn hash<H>(&self, state: &mut H)
        where H: Hasher
    {
        self.path.hash(state);
    }
}