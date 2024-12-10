use aoc_runner_derive::{aoc, aoc_generator};

type Int = isize;

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<Int> {
    let mut res = vec![];

    let mut file_id = 0;

    for (i, c) in input.chars().enumerate() {
        match i % 2 {
            0 => {
                res.extend(vec![file_id; c.to_digit(10).unwrap() as usize]);
                file_id += 1
            }
            1 => {
                res.extend(vec![-1; c.to_digit(10).unwrap() as usize]);
            }
            _ => unreachable!(),
        }
    }

    res
}

#[aoc(day9, part1)]
fn part_one(input: &Vec<Int>) -> Int {
    let mut res = 0;

    let mut i: usize = 0;
    let mut j: usize = input.len() - 1;

    while i <= j {
        while input[j] < 0 {
            j -= 1;
        }

        if j < i {
            break;
        }

        if input[i] < 0 {
            res += input[j] * i as Int;
            j -= 1;
        } else {
            res += input[i] * i as Int
        }

        i += 1;
    }

    res
}

#[aoc(day9, part2)]
fn part_two(input: &Vec<Int>) -> Int {
    0
}
