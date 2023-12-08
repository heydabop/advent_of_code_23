use regex::Regex;
use std::collections::HashMap;

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
    println!("part1: {steps}");

    let mut starts = vec![];
    for key in map.keys() {
        if key.ends_with('A') {
            starts.push(key)
        }
    }
    let steps: Vec<u64> = starts
        .into_iter()
        .map(|mut pos| {
            let mut steps = 0;
            'outer: loop {
                for &instruction in instructions {
                    let next = &map[pos];
                    match instruction {
                        b'L' => pos = &next.0,
                        b'R' => pos = &next.1,
                        _ => panic!("unknown instruction {instruction}"),
                    }
                    steps += 1;
                    if pos.ends_with('Z') {
                        break 'outer;
                    }
                }
            }
            steps
        })
        .collect();
    let mut synced_steps = steps[0];
    for &step in &steps[1..] {
        synced_steps = lcm(synced_steps, step);
    }

    println!("part2: {synced_steps}");
}

fn gcd(mut x: u64, mut y: u64) -> u64 {
    // Euclidean gcd algorithm
    // assumes x > 0 && y > 0
    while x != y {
        if x > y {
            x -= y;
        } else {
            y -= x;
        }
    }
    x
}

fn lcm(x: u64, y: u64) -> u64 {
    (x / gcd(x, y)) * y
}
