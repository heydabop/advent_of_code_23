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
    let mut part_1_total = 0;
    let mut part_2_total = 0;
    for line in input.lines() {
        let mut game_max_red = 0;
        let mut game_max_green = 0;
        let mut game_max_blue = 0;

        let game_number: u32 = regex_get_u32(&game_regex, line);
        let grabs = line.split(';');
        let mut valid = true;
        for grab in grabs {
            let red = regex_get_u32(&red_regex, grab);
            game_max_red = game_max_red.max(red);
            if red > max_red {
                valid = false;
            }
            let green = regex_get_u32(&green_regex, grab);
            game_max_green = game_max_green.max(green);
            if green > max_green {
                valid = false;
            }
            let blue = regex_get_u32(&blue_regex, grab);
            game_max_blue = game_max_blue.max(blue);
            if blue > max_blue {
                valid = false;
            }
        }

        if valid {
            part_1_total += game_number;
        }

        part_2_total += game_max_red as u64 * game_max_green as u64 * game_max_blue as u64;
    }
    println!("part 1: {part_1_total}");
    println!("part 2: {part_2_total}");
}

fn regex_get_u32(re: &Regex, s: &str) -> u32 {
    if let Some(caps) = re.captures(s) {
        caps.get(1).unwrap().as_str().parse().unwrap()
    } else {
        0
    }
}
