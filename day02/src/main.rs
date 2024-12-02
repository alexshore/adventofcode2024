use std::i32;

fn main() {
    let input = include_str!("../input.txt").trim_end();
    println!("{}", part_one(input));
    println!("{}", part_two(input))
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|val| val.parse().unwrap())
                .collect()
        })
        .collect()
}

enum ReportState {
    Increasing,
    Decreasing,
    Unseen,
}

fn part_one(input: &str) -> String {
    let reports = parse_input(input);

    reports
        .iter()
        .filter(|&report| {
            let mut state = ReportState::Unseen;
            for i in 0..report.len() - 1 {
                match i32::abs_diff(report[i], report[i + 1]) {
                    1..=3 => (),
                    _ => return false,
                }

                match (&state, report[i] - report[i + 1]) {
                    (ReportState::Unseen, diff) if diff < 0 => state = ReportState::Increasing,
                    (ReportState::Unseen, diff) if diff > 0 => state = ReportState::Decreasing,
                    (ReportState::Decreasing, diff) if diff < 0 => return false,
                    (ReportState::Increasing, diff) if diff > 0 => return false,
                    _ => (),
                }
            }
            true
        })
        .count()
        .to_string()
}

fn part_two(input: &str) -> String {
    todo!()
}
