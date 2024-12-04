use aoc_runner_derive::{aoc, aoc_generator};

type Lists = (Vec<u32>, Vec<u32>);

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Lists {
    let mut left = vec![];
    let mut right = vec![];

    input.lines().for_each(|line| {
        let mut split_line = line.split_whitespace();
        let left_val = split_line.next().unwrap().parse::<u32>().unwrap();
        let right_val = split_line.next().unwrap().parse::<u32>().unwrap();

        left.push(left_val);
        right.push(right_val);
    });

    left.sort();
    right.sort();

    (left, right)
}

#[aoc(day1, part1)]
fn part_one(input: &Lists) -> u32 {
    input
        .0
        .iter()
        .zip(input.1.iter())
        .fold(0, |acc, (x, y)| acc + x.abs_diff(*y))
}

#[aoc(day1, part2)]
fn part_two(input: &Lists) -> u32 {
    input.0.iter().fold(0, |acc, a| {
        acc + (input.1.iter().filter(|&b| *b == *a).count() as u32 * *a)
    })
}
