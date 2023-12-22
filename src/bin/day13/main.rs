use std::fmt;

struct Pattern {
    lines: Vec<Vec<bool>>,
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.lines {
            for b in line {
                if *b {
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

impl Pattern {
    fn reflection_value(&self) -> u64 {
        // rows
        for i in 1..self.lines.len() {
            if self.reflects_row(i) {
                return 100 * (i as u64);
            }
        }
        for i in 1..self.lines[0].len() {
            if self.reflects_column(i) {
                return i as u64;
            }
        }
        panic!("doesn't reflect");
    }

    fn reflects_row(&self, index: usize) -> bool {
        self.lines[..index]
            .iter()
            .rev()
            .zip(self.lines[index..].iter())
            .all(|(top, bottom)| top == bottom)
    }

    fn reflects_column(&self, index: usize) -> bool {
        (0..index)
            .rev()
            .zip(index..self.lines[0].len())
            .all(|(left, right)| {
                for i in 0..self.lines.len() {
                    if self.lines[i][left] != self.lines[i][right] {
                        return false;
                    }
                }
                true
            })
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut patterns: Vec<Pattern> = vec![];
    let mut pattern: Vec<Vec<bool>> = vec![];
    for line in input.lines() {
        if line.is_empty() {
            patterns.push(Pattern { lines: pattern });
            pattern = vec![];
            continue;
        }
        pattern.push(line.bytes().map(|b| b == b'#').collect());
    }
    patterns.push(Pattern { lines: pattern });
    println!(
        "part 1: {}",
        patterns
            .into_iter()
            .fold(0, |acc, p| acc + p.reflection_value())
    );
}
