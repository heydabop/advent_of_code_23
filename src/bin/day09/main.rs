fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let histories: Vec<Vec<i64>> = input
        .lines()
        .map(|line| line.split(' ').map(|v| v.parse().unwrap()).collect())
        .collect();
    let total = histories.into_iter().fold(0, |acc, history| {
        let expanded = expand(history);
        acc + expanded.last().unwrap()
    });
    println!("{total}");
}

fn expand(mut history: Vec<i64>) -> Vec<i64> {
    let mut diff: Vec<i64> = history.windows(2).map(|v| v[1] - v[0]).collect();
    let expanded = if diff.iter().all(|&i| i == 0) {
        diff.push(0);
        diff
    } else {
        expand(diff)
    };
    history.push(history.last().unwrap() + expanded.last().unwrap());
    history
}
