use std::collections::HashMap;

use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().as_bytes();
    lines.next().unwrap();
    let mut map = HashMap::new();
    let map_line_regex = Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();
    for line in lines {
        let caps = map_line_regex.captures(line).unwrap();
        map.insert(caps[1].to_owned(), (caps[2].to_owned(), caps[3].to_owned()));
    }
    let mut steps = 0;
    let start = String::from("AAA");
    let mut pos = &start;
    'outer: loop {
        for &instruction in instructions {
            let next = &map[pos];
            match instruction {
                b'L' => pos = &next.0,
                b'R' => pos = &next.1,
                _ => panic!("unknown instruction {instruction}"),
            }
            steps += 1;
            if pos == "ZZZ" {
                break 'outer;
            }
        }
    }
    println!("{steps}");
}
