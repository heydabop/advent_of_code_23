use std::fmt;

struct StarMap {
    map: Vec<Vec<Option<u16>>>, // Some(val) indicates a galaxy, numbered for ID
    galaxies: Vec<(u64, u64)>,  // coordinates of galaxies in map, indxed by galaxy number
    empty_rows: Vec<u64>,       // rows with no galaxies in them
    empty_cols: Vec<u64>,       // columns with no galaxies in them
    expansion_factor: u32,      // how many "extra" rows/columns an empty row/column is
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
    fn new(input: &str, expansion_factor: u32) -> Self {
        // read map from input, incrementing galaxy IDs and marking empty rows along the way
        let mut num_galaxies = 0;
        let mut empty_rows = vec![];
        let mut empty_cols = vec![];
        let map: Vec<Vec<Option<u16>>> = input
            .lines()
            .enumerate()
            .map(|(i, line)| {
                let mut empty = true;
                // construct row, recording if its empty
                let row = line
                    .chars()
                    .map(|c| {
                        if c == '#' {
                            empty = false;
                            num_galaxies += 1;
                            Some(num_galaxies - 1)
                        } else {
                            None
                        }
                    })
                    .collect();
                if empty {
                    empty_rows.push(u64::try_from(i).unwrap());
                }
                row
            })
            .collect();

        // check for empty columns
        for i in 0..map.len() {
            let mut empty = true;
            for row in &map {
                if row[i].is_some() {
                    // if any row in this column has a galaxy, mark it and stop looping
                    empty = false;
                    break;
                }
            }
            if empty {
                empty_cols.push(u64::try_from(i).unwrap());
            }
        }

        // find and store unexpanded coordinates of galaxies
        let mut galaxies = vec![(0, 0); num_galaxies as usize];
        for (y, row) in map.iter().enumerate() {
            for (x, g) in row.iter().enumerate() {
                if let Some(id) = g {
                    galaxies[*id as usize] = (u64::try_from(y).unwrap(), u64::try_from(x).unwrap());
                }
            }
        }

        Self {
            map,
            galaxies,
            empty_rows,
            empty_cols,
            expansion_factor: expansion_factor - 1, // leaving this as is means we double-count the original row during path finding, so subtract 1 here
        }
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
            let mut sorted_y = [root.0, *y];
            sorted_y.sort();
            let mut sorted_x = [root.1, *x];
            sorted_x.sort();
            // first get normal distances between galaxies
            let unexpanded_y = sorted_y[1] - sorted_y[0];
            let unexpanded_x = sorted_x[1] - sorted_x[0];
            // then find how many empty columns and rows are between the two, and increase distance using expansion factor
            let mut num_empty_rows = 0;
            for &empty_row in &self.empty_rows {
                // row is between galaxies
                if empty_row > sorted_y[0] && empty_row < sorted_y[1] {
                    num_empty_rows += 1;
                }
            }
            let mut num_empty_cols = 0;
            for &empty_col in &self.empty_cols {
                // column is between galaxies
                if empty_col > sorted_x[0] && empty_col < sorted_x[1] {
                    num_empty_cols += 1;
                }
            }
            let dist_y = unexpanded_y + num_empty_rows * self.expansion_factor as u64;
            let dist_x = unexpanded_x + num_empty_cols * self.expansion_factor as u64;
            dists[i] = dist_y + dist_x;
        }
        dists
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let map1 = StarMap::new(&input, 2);
    println!("part 1: {}", map1.shortest_paths_between_all_pairs());
    let map2 = StarMap::new(&input, 1000000);
    println!("part 2: {}", map2.shortest_paths_between_all_pairs());
}
