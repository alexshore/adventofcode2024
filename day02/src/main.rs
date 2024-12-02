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
    Unknown,
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

fn part_one(input: &str) -> usize {
    let reports = parse_input(input);

    reports
        .iter()
        .filter(|&report| is_report_safe(report))
        .count()
}

fn part_two(input: &str) -> usize {
    let reports = parse_input(input);

    reports
        .iter()
        .filter(|&report| {
            // if report isn't safe by default then we'll check with the "Problem Dampener"
            if !is_report_safe(report) {
                for i in 0..report.len() {
                    // exclude each item in turn to see if there is still a safe configuration
                    if is_report_safe(&[&report[..i], &report[i + 1..]].concat()) {
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
