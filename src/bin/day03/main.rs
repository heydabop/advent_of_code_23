struct Scanner {
    number_start: Option<usize>,
    number_bytes: Vec<u8>,
    total: u64,
}

impl Scanner {
    fn new() -> Self {
        Self {
            number_start: None,
            number_bytes: vec![],
            total: 0,
        }
    }

    fn scan(&mut self, input: String) -> u64 {
        let schematic: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
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
                    self.assemble_and_check_number(&schematic, x_start, x, y);
                }
            }

            // assemble and check number if one is currently being built as line ends
            if let Some(x_start) = self.number_start {
                self.assemble_and_check_number(&schematic, x_start, line.len() - 1, y);
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
        // check if current byte is a symbol
        if is_symbol(schematic[y][x]) {
            self.add_part();
            return;
        }
        // check if byte before number is a symbol
        if x_start > 0 && is_symbol(schematic[y][x_start]) {
            self.add_part();
            return;
        }
        // check row above for symbols
        if y > 0 {
            for i in x_start..=x {
                if is_symbol(schematic[y - 1][i]) {
                    self.add_part();
                    return;
                }
            }
        }
        // check row after for symbols
        if y + 1 < schematic.len() {
            for i in x_start..=x {
                if is_symbol(schematic[y + 1][i]) {
                    self.add_part();
                    return;
                }
            }
        }
        self.clear_number();
    }

    fn add_part(&mut self) {
        let number = std::str::from_utf8(&self.number_bytes)
            .unwrap()
            .parse::<u64>()
            .unwrap();
        self.total += number;
        self.clear_number();
    }

    fn clear_number(&mut self) {
        self.number_bytes = vec![];
        self.number_start = None;
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut scanner = Scanner::new();
    println!("{}", scanner.scan(input));
}

fn is_symbol(byte: u8) -> bool {
    if byte == b'.' {
        return false;
    }
    if byte.is_ascii_digit() {
        return false;
    }
    true
}
