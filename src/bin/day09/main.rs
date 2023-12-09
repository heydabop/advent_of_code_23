#![feature(iter_map_windows)]

use std::collections::VecDeque;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let histories: Vec<Vec<i64>> = input
        .lines()
        .map(|line| line.split(' ').map(|v| v.parse().unwrap()).collect())
        .collect();
    let total = histories.clone().into_iter().fold(0, |acc, history| {
        let expanded = expand(history);
        acc + expanded.last().unwrap()
    });
    println!("part 1: {total}");

    let back_total = histories.into_iter().fold(0, |acc, history| {
        let expanded = expand_back(history.into());
        acc + expanded.front().unwrap()
    });
    println!("part 2: {back_total}");
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

fn expand_back(mut history: VecDeque<i64>) -> VecDeque<i64> {
    let mut diff: VecDeque<i64> = history
        .iter()
        .map_windows(|[&prev, &curr]| curr - prev)
        .collect();
    let expanded = if diff.iter().all(|&i| i == 0) {
        diff.push_front(0);
        diff
    } else {
        expand_back(diff)
    };
    history.push_front(history.front().unwrap() - expanded.front().unwrap());
    history
}
