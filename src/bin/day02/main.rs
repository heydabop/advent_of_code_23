use regex::Regex;

fn main() {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let game_regex = Regex::new(r"Game (\d+):").unwrap();
    let red_regex = Regex::new(r"(\d+) red").unwrap();
    let blue_regex = Regex::new(r"(\d+) blue").unwrap();
    let green_regex = Regex::new(r"(\d+) green").unwrap();
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut total = 0;
    'outer: for line in input.lines() {
        let game_number: u32 = regex_get_u32(&game_regex, line);
        let grabs = line.split(';');
        for grab in grabs {
            let red = regex_get_u32(&red_regex, grab);
            if red > max_red {
                continue 'outer;
            }
            let green = regex_get_u32(&green_regex, grab);
            if green > max_green {
                continue 'outer;
            }
            let blue = regex_get_u32(&blue_regex, grab);
            if blue > max_blue {
                continue 'outer;
            }
        }
        total += game_number;
    }
    println!("{total}");
}

fn regex_get_u32(re: &Regex, s: &str) -> u32 {
    if let Some(caps) = re.captures(s) {
        caps.get(1).unwrap().as_str().parse().unwrap()
    } else {
        0
    }
}
