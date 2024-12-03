use aoc_runner_derive::{aoc, aoc_generator};

struct Report {
    values: Vec<i32>,
}

impl FromIterator<i32> for Report {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        let mut values = vec![];

        for item in iter {
            values.push(item)
        }

        Self { values }
    }
}

type Reports = Vec<Report>;

enum ReportState {
    Increasing,
    Decreasing,
    Unknown,
}

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Reports {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|val| val.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_report_safe(report: &[i32]) -> bool {
    let mut state = ReportState::Unknown;
    for i in 0..report.len() - 1 {
        // just check if the diff is within the allowed range
        match i32::abs_diff(report[i], report[i + 1]) {
            1..=3 => (),
            _ => return false,
        }

        // can't fail on the first two items in the report, we just find out whether its increasing or decreasing.
        // on 2-3, 3-4, 4-5, etc. we check behaviour compared to the expected behaviour and fail if bad
        match &state {
            ReportState::Unknown if report[i] < report[i + 1] => state = ReportState::Increasing,
            ReportState::Unknown if report[i] > report[i + 1] => state = ReportState::Decreasing,
            ReportState::Decreasing if report[i] < report[i + 1] => return false,
            ReportState::Increasing if report[i] > report[i + 1] => return false,
            _ => (),
        }
    }
    true
}

#[aoc(day2, part1)]
fn part_one(reports: &Reports) -> usize {
    reports
        .iter()
        .filter(|&report| is_report_safe(&report.values))
        .count()
}

#[aoc(day2, part2)]
fn part_two(reports: &Reports) -> usize {
    reports
        .iter()
        .filter(|&report| {
            // if report isn't safe by default then we'll check with the "Problem Dampener"
            if !is_report_safe(&report.values) {
                for i in 0..report.values.len() {
                    // exclude each item in turn to see if there is still a safe configuration
                    if is_report_safe(&[&report.values[..i], &report.values[i + 1..]].concat()) {
                        return true;
                    }
                }
                // we haven't found a safe configuration so filter it out
                return false;
            }
            // report is safe by default :)
            true
        })
        .count()
}
