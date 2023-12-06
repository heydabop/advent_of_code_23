fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();
    let time_line = lines.next().unwrap();
    let times: Vec<u64> = time_line[time_line.find(':').unwrap() + 1..]
        .split(' ')
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    let dist_line = lines.next().unwrap();
    let dists: Vec<u64> = dist_line[dist_line.find(':').unwrap() + 1..]
        .split(' ')
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    let options: Vec<u64> = times
        .iter()
        .enumerate()
        .map(|(i, time)| {
            let goal_dist = dists[i];
            let mut wins = 0;
            for hold_ms in 1..*time {
                let move_ms = time - hold_ms;
                let dist = move_ms * hold_ms;
                if dist > goal_dist {
                    wins += 1;
                }
            }
            wins
        })
        .collect();
    println!("part 1: {}", options.into_iter().product::<u64>());

    let time: u64 = times
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join("")
        .parse()
        .unwrap();
    let goal_dist: u64 = dists
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join("")
        .parse()
        .unwrap();
    let mut wins = 0;
    for hold_ms in 1..time {
        let move_ms = time - hold_ms;
        let dist = move_ms * hold_ms;
        if dist > goal_dist {
            wins += 1;
        }
    }
    println!("part 2: {wins}");
}
