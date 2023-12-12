use std::{collections::VecDeque, fmt};

struct StarMap {
    map: Vec<VecDeque<Option<u16>>>, // Some(val) indicates a galaxy, numbered for ID
    galaxies: Vec<(i64, i64)>,       // coordinates of galaxies in map, indxed by galaxy number
}

impl fmt::Display for StarMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.map {
            for c in row {
                if c.is_some() {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl StarMap {
    fn new(input: &str) -> Self {
        // read map from input, true indicates a galaxy
        let unexpanded_map: Vec<Vec<bool>> = input
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();
        let mut num_galaxies = 0;
        let mut map: Vec<VecDeque<Option<u16>>> = vec![];
        for row in unexpanded_map.into_iter() {
            let double = row.iter().all(|&g| !g);
            // any row that's empty gets expanded
            if double {
                map.push(vec![None; row.len()].into());
                map.push(vec![None; row.len()].into());
                continue;
            }
            // otherwise, insert a row, ensuring to increment galaxy IDs
            map.push(
                row.iter()
                    .map(|g| {
                        if *g {
                            num_galaxies += 1;
                            Some(num_galaxies - 1)
                        } else {
                            None
                        }
                    })
                    .collect(),
            );
        }

        // manually iterate through map, since modifying map while using an iter on it seems unwise/impossible
        // effectively `for i in 0..cols` but i and cols will grow
        let mut cols = map[0].len();
        let mut i = 0;
        loop {
            let mut empty = true;
            for row in &map {
                if row[i].is_some() {
                    // if any row in this column has a galaxy, mark it and stop looping
                    empty = false;
                    break;
                }
            }
            // if this column is empty, insert another empty column before this one
            if empty {
                for row in map.iter_mut() {
                    row.insert(i, None);
                }
                // increment both our index counter and the total length of the map
                i += 1;
                cols += 1;
            }
            // normal loop increment and range check
            i += 1;
            if i == cols {
                break;
            }
        }

        // now that map is expanded, find and store coordinates of galaxies
        let mut galaxies = vec![(0, 0); num_galaxies as usize];
        for (y, row) in map.iter().enumerate() {
            for (x, g) in row.iter().enumerate() {
                if let Some(id) = g {
                    galaxies[*id as usize] = (i64::try_from(y).unwrap(), i64::try_from(x).unwrap());
                }
            }
        }

        Self { map, galaxies }
    }

    // return the sum of all paths between every pair of galaxies
    pub fn shortest_paths_between_all_pairs(&self) -> u64 {
        let mut total = 0;
        for i in 0..self.galaxies.len() {
            let paths = self.shortest_paths_from_galaxy(i);
            total = paths.into_iter().fold(total, |total, path| total + path);
        }
        total
    }

    // return vec of path distances from galaxy to each galaxy with an ID greater than it
    // so galaxy 1 with 5 galaxies total will have a vec of len 3, for galaxies 2, 3, and 4
    fn shortest_paths_from_galaxy(&self, galaxy: usize) -> Vec<u64> {
        let mut dists = vec![0; self.galaxies.len() - galaxy - 1];
        let root = self.galaxies[galaxy];
        for (i, (y, x)) in self.galaxies[galaxy + 1..].iter().enumerate() {
            dists[i] = u64::try_from((root.0 - y).abs() + (root.1 - x).abs()).unwrap();
        }
        dists
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let map = StarMap::new(&input);
    println!("part 1: {}", map.shortest_paths_between_all_pairs());
}
