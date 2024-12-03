use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Debug)]
struct List {
    left: Vec<u32>,
    right: Vec<u32>,
}

impl List {
    fn new() -> Self {
        Self {
            left: vec![],
            right: vec![],
        }
    }
}

#[aoc_generator(day1)]
fn parse_input(input: &str) -> List {
    let mut list = List::new();

    input.lines().for_each(|line| {
        let mut split_line = line.split_whitespace();
        let left_val = split_line.next().unwrap().parse::<u32>().unwrap();
        let right_val = split_line.next().unwrap().parse::<u32>().unwrap();

        list.left.push(left_val);
        list.right.push(right_val);
    });

    list
}

#[aoc(day1, part1)]
fn part_one(input: &List) -> u32 {
    let mut list = input.clone();

    list.left.sort();
    list.right.sort();

    list.left
        .into_iter()
        .zip(list.right)
        .fold(0, |acc, (x, y)| acc + x.abs_diff(y))
}

#[aoc(day1, part2)]
fn part_two(input: &List) -> u32 {
    let mut list = input.clone();

    list.left.sort_unstable();
    list.left.dedup();

    list.left.iter().fold(0, |acc, a| {
        acc + (list.right.iter().filter(|&b| *b == *a).count() as u32 * *a)
    })
}
