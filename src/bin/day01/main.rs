const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    // initially thought about doing a sliding window over byte slices to find numbers
    // instead decided to be lazy and just .find the number substrings
    /*let numbers = [
        vec!['o', 'n', 'e'],
        vec!['t', 'w', 'o'],
        vec!['t', 'h', 'r', 'e', 'e'],
        vec!['f', 'o', 'u', 'r'],
        vec!['f', 'i', 'v', 'e'],
        vec!['s', 'i', 'x'],
        vec!['s', 'e', 'v', 'e', 'n'],
        vec!['e', 'i', 'g', 'h', 't'],
        vec!['n', 'i', 'n', 'e'],
    ];*/
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut total: u64 = 0;
    for line in input.lines() {
        let mut positions = [usize::MAX; 9];
        // find the first place each string number appears in this line
        for (i, s) in NUMBERS.iter().enumerate() {
            if let Some(pos) = line.find(s) {
                positions[i] = pos;
            }
        }
        let bytes = line.as_bytes();
        // find the first place a numeral appears in this line
        let mut first_position = usize::MAX;
        let mut first = 0;
        for (i, b) in bytes.iter().enumerate() {
            if is_num(*b) {
                first_position = i;
                first = *b - 48;
                break;
            }
        }
        for (i, pos) in positions.iter().enumerate() {
            if *pos < first_position {
                first_position = *pos;
                first = i as u8 + 1; // 0th index is one, 1st index is two, etc
            }
        }
        total += first as u64 * 10;

        // do it all again backwards
        positions = [0; 9];
        for (i, s) in NUMBERS.iter().enumerate() {
            if let Some(pos) = line.rfind(s) {
                positions[i] = pos;
            }
        }
        let mut last_position = 0;
        let mut last = 0;
        for (i, b) in bytes.iter().enumerate().rev() {
            if is_num(*b) {
                last_position = i;
                last = *b - 48;
                break;
            }
        }
        for (i, pos) in positions.iter().enumerate() {
            if *pos > last_position {
                last_position = *pos;
                last = i as u8 + 1; // 0th index is one, 1st index is two, etc
            }
        }
        total += last as u64;
    }
    println!("{total}");
}

fn is_num(ascii_byte: u8) -> bool {
    (48..=57).contains(&ascii_byte)
}
