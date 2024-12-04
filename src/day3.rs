use aoc_runner_derive::aoc;
use regex::Regex;

#[aoc(day3, part1)]
fn part_one(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [a, b])| a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap())
        .sum()
}

#[derive(PartialEq, Eq)]
enum State {
    Enabled,
    Disabled,
}

#[aoc(day3, part2)]
fn part_two(input: &str) -> u32 {
    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    let nums = Regex::new(r"(\d+),(\d+)").unwrap();

    let mut state = State::Enabled;
    let mut res = 0;

    re.find_iter(input).for_each(|m| match m.as_str() {
        "do()" => state = State::Enabled,
        "don't()" => state = State::Disabled,
        operation => {
            if state == State::Enabled {
                let (_, [a, b]) = nums.captures(&operation).unwrap().extract();
                res += a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap()
            }
        }
    });
    res
}
