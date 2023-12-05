fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut total = 0;
    for line in input.lines() {
        let mut matches = 0;
        let start = line.find(':').unwrap();
        let mid = line.find('|').unwrap();
        let winning_numbers: Vec<u64> = line[start + 1..mid]
            .split(' ')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        line[mid + 1..].split(' ').for_each(|s| {
            if let Ok(num) = s.trim().parse() {
                if winning_numbers.contains(&num) {
                    matches += 1;
                }
            }
        });
        if matches > 0 {
            total += 2_u64.pow(matches - 1);
        }
    }
    println!("{total}");
}
