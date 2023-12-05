mod part1;
mod part2;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let schematic: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    println!("part1: {}", part1::Part1::default().scan(&schematic));
    println!("part2: {}", part2::Part2::default().scan(&schematic));
}
