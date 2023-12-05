use std::collections::HashMap;

#[derive(Default)]
pub struct Part2 {
    number_start: Option<usize>,
    number_bytes: Vec<u8>,
    number: u64,
    // map of y,x gear indices and the numbers adjacent too them
    gear_numbers: HashMap<(usize, usize), Vec<u64>>,
    total: u64,
}

impl Part2 {
    // scan schematic, parsing out numbers as they're encounted
    // once a number is found, check adjacent spaces for a gear (*)
    // if a gear is found adjacenet to that number, record that number in gear_numbers, using gear coordinates as the tupe
    // then add up the product of all number pairs adjacnet to a gear
    pub fn scan(&mut self, schematic: &[&[u8]]) -> u64 {
        for (y, line) in schematic.iter().enumerate() {
            for (x, byte) in line.iter().enumerate() {
                if byte.is_ascii_digit() {
                    if self.number_start.is_none() {
                        // if we arent currently reading a number, mark the start before this number
                        self.number_start = if x > 0 { Some(x - 1) } else { Some(0) };
                    }
                    // add digit to vec to be assembled into number at number end
                    self.number_bytes.push(*byte);
                } else if let Some(x_start) = self.number_start {
                    // if we were building a number and have hit a non-digit char, assemble number and check adjacency
                    self.assemble_and_check_number(schematic, x_start, x, y);
                }
            }

            // assemble and check number if one is currently being built as line ends
            if let Some(x_start) = self.number_start {
                self.assemble_and_check_number(schematic, x_start, line.len() - 1, y);
            }
        }
        // go through all numbers that were adjacent to a gear, only adding those for which exactly two were adjacent to a given gear
        for numbers in self.gear_numbers.values() {
            if numbers.len() == 2 {
                self.total += numbers[0] * numbers[1];
            }
        }
        self.total
    }

    fn assemble_and_check_number(
        &mut self,
        schematic: &[&[u8]],
        x_start: usize,
        x: usize,
        y: usize,
    ) {
        self.number = std::str::from_utf8(&self.number_bytes)
            .unwrap()
            .parse::<u64>()
            .unwrap();
        // check if current byte is a gear
        if schematic[y][x] == b'*' {
            self.register_number(y, x);
        }
        // check if byte before number is a gear
        if x_start > 0 && schematic[y][x_start] == b'*' {
            self.register_number(y, x_start);
        }
        // check row above for gears
        if y > 0 {
            for i in x_start..=x {
                if schematic[y - 1][i] == b'*' {
                    self.register_number(y - 1, i);
                }
            }
        }
        // check row after for gears
        if y + 1 < schematic.len() {
            for i in x_start..=x {
                if schematic[y + 1][i] == b'*' {
                    self.register_number(y + 1, i);
                }
            }
        }
        self.clear_number();
    }

    fn register_number(&mut self, y: usize, x: usize) {
        self.gear_numbers
            .entry((y, x))
            .or_default()
            .push(self.number);
    }

    fn clear_number(&mut self) {
        self.number_bytes = vec![];
        self.number_start = None;
        self.number = 0;
    }
}
