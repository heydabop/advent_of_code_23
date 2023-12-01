fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut total: u64 = 0;
    for line in input.lines() {
        let bytes = line.as_bytes();
        for b in bytes {
            if *b >= 48 && *b <= 57 {
                total += ((*b - 48) as u64) * 10;
                break;
            }
        }
        for b in bytes.iter().rev() {
            if *b >= 48 && *b <= 57 {
                total += (*b - 48) as u64;
                break;
            }
        }
    }
    println!("{total}");
}
