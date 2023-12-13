#[derive(Debug)]
struct Row {
    springs: Vec<Option<bool>>, // true is operational, false is damaged, None is unknown
    groups: Vec<u32>,           // contiguous groups of damaged springs
}

impl Row {
    pub fn from_str(line: &str) -> Self {
        let s: Vec<_> = line.split(' ').collect();
        let springs = s[0]
            .chars()
            .map(|c| match c {
                '.' => Some(true),
                '#' => Some(false),
                '?' => None,
                _ => panic!("unexpected char {c}"),
            })
            .collect();
        let groups = s[1].split(',').map(|n| n.parse().unwrap()).collect();
        Self { springs, groups }
    }

    pub fn possible_arrangements(&self) -> u64 {
        // get all possible arrangments
        let arrangments = Self::generate_possible_springs(&self.springs);
        let mut num_valid = 0;
        // test every arrangment to see if if matches self.groups
        for a in arrangments {
            let mut group_idx = 0;
            let mut current_group_len = 0;
            let mut valid = true;
            for s in a {
                if !s {
                    // count how many broken springs weve seen in a row
                    current_group_len += 1;
                } else if current_group_len != 0 {
                    // if this spring isnt broken and we were counting springs, check to see if our group length matches the expected length
                    if let Some(group_len) = self.groups.get(group_idx) {
                        if current_group_len == *group_len {
                            // group matches, reset counter and increment to next group
                            current_group_len = 0;
                            group_idx += 1;
                        } else {
                            // group length doesnt match, break loop
                            valid = false;
                            break;
                        }
                    } else {
                        // we have more groups in this arrangement than expect, break loop
                        valid = false;
                        break;
                    }
                }
            }
            // two cases to consider at end of loop
            // either we were counting a group, in which case we need to check if its length matches an expected length and that its the last group
            // or we werent counting a group, and we need to check that we accounted for every expected group
            if (current_group_len != 0
                && (group_idx != self.groups.len() - 1
                    || current_group_len != self.groups[group_idx]))
                || (current_group_len == 0 && group_idx != self.groups.len())
            {
                valid = false;
            }
            if valid {
                num_valid += 1;
            }
        }
        num_valid
    }

    // recursively generates every possible springs row based on unknown elements
    fn generate_possible_springs(springs: &[Option<bool>]) -> Vec<Vec<bool>> {
        // generate first element of row
        let heads = if let Some(spring) = springs[0] {
            // either copying existing element if known
            vec![vec![spring]]
        } else {
            // or accounting for both possibilities if unknown
            vec![vec![true], vec![false]]
        };
        let tails = if springs.len() == 1 {
            // were at the end of the row, return
            return heads;
        } else {
            // otherwise, recursively generate the rest of the row
            Self::generate_possible_springs(&springs[1..])
        };
        let mut possible = vec![];
        // combine possible first elements and rest of rows
        for t in &tails {
            for h in &heads {
                possible.push([h.clone(), t.clone()].concat());
            }
        }
        possible
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let rows: Vec<_> = input.lines().map(Row::from_str).collect();
    println!(
        "part 1: {}",
        rows.iter()
            .fold(0, |acc, r| acc + r.possible_arrangements())
    );
}
