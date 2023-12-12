// this solution feels ugly and im not a huge fan of it, but it works

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        use Direction::*;
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy)]
struct Node {
    pub pipe: u8,
    pub main_loop: bool,
}

impl Node {
    fn new(pipe: u8) -> Self {
        Self {
            pipe,
            main_loop: false,
        }
    }

    fn directions(&self) -> Option<[Direction; 2]> {
        use Direction::*;
        match self.pipe {
            b'|' => Some([North, South]),
            b'-' => Some([East, West]),
            b'L' => Some([North, East]),
            b'J' => Some([North, West]),
            b'7' => Some([South, West]),
            b'F' => Some([South, East]),
            b'.' => None,
            b'S' => None,
            _ => panic!("unexpected {}", self.pipe),
        }
    }
}

struct Map {
    pub start: Point,
    pub grid: Vec<Vec<Node>>,
}

impl Map {
    pub fn find_furthest_dist(&mut self) -> usize {
        use Direction::*;
        let mut pos = self.start;
        self.grid[pos.y][pos.x].main_loop = true;
        let mut dir = if self.traverse(pos, North).1.is_some() {
            North
        } else if self.traverse(pos, East).1.is_some() {
            East
        } else if self.traverse(pos, West).1.is_some() {
            West
        } else {
            panic!("unable to start loop at {pos:?}");
        };
        let start_dir = dir;
        let end_dir;
        let mut length = 0;
        let furthest = loop {
            let (p, d) = self.traverse(pos, dir);
            pos = p.unwrap();
            length += 1;
            if pos == self.start {
                end_dir = dir.opposite();
                break length / 2;
            }
            self.grid[pos.y][pos.x].main_loop = true;
            dir = d.unwrap();
        };
        self.grid[self.start.y][self.start.x].pipe = match (start_dir, end_dir) {
            (North, South) => b'|',
            (North, East) => b'L',
            (North, West) => b'J',
            (East, West) => b'-',
            (East, South) => b'F',
            (West, South) => b'7',
            _ => panic!("unexpected dir pair {start_dir:?}, {end_dir:?}"),
        };
        furthest
    }

    pub fn find_num_contained_nodes(&self) -> usize {
        let mut nodes = 0;
        for row in &self.grid {
            let mut in_loop = false;
            for node in row {
                let dirs = node.directions();
                if node.main_loop {
                    if dirs.unwrap().contains(&Direction::North) {
                        in_loop = !in_loop;
                    }
                } else if in_loop {
                    nodes += 1;
                }
            }
        }
        nodes
    }

    fn traverse(&self, mut pos: Point, mut dir: Direction) -> (Option<Point>, Option<Direction>) {
        use Direction::*;
        match dir {
            North => {
                if pos.y == 0 {
                    return (None, None);
                }
                pos.y -= 1;
            }
            East => pos.x += 1,
            South => pos.y += 1,
            West => {
                if pos.x == 0 {
                    return (None, None);
                }
                pos.x -= 1;
            }
        };
        let Some(node) = self.get_node(pos) else {
            return (None, None);
        };
        let Some(next_dirs) = node.directions() else {
            return (Some(pos), None);
        };
        if !next_dirs.contains(&dir.opposite()) {
            return (Some(pos), None);
        }
        dir = *next_dirs.iter().find(|&&d| d != dir.opposite()).unwrap();
        (Some(pos), Some(dir))
    }

    fn get_node(&self, pos: Point) -> Option<&Node> {
        self.grid.get(pos.y).map(|r| r.get(pos.x))?
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut start = Point { x: 0, y: 0 };
    let grid: Vec<Vec<Node>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(x, &pipe)| {
                    if pipe == b'S' {
                        start = Point { x, y };
                    }
                    Node::new(pipe)
                })
                .collect()
        })
        .collect();
    let mut map = Map { start, grid };
    println!("part 1: {}", map.find_furthest_dist());
    println!("part 2: {}", map.find_num_contained_nodes());
}
