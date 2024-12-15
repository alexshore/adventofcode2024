use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

fn get_start_pos(input: &str) -> Position {
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.char_indices() {
            if c == '^' {
                return (i as Int, j as Int);
            }
        }
    }
    unreachable!()
}

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Input {
    Input {
        grid: input.lines().map(|line| line.chars().collect()).collect(),
        start_pos: get_start_pos(input),
    }
}

struct Input {
    grid: Grid,
    start_pos: Position,
}

type Int = isize;
type Grid = Vec<Vec<char>>;
type Position = (Int, Int);

fn in_bounds(grid: &Grid, pos: Position) -> bool {
    pos.0 > 0 && pos.1 > 0 && pos.0 < grid.len() as isize && pos.1 < grid[0].len() as isize
}

#[aoc(day6, part1)]
fn part_one(input: &Input) -> Int {
    let mut dir: Position = (-1, 0);
    let mut pos: Position = input.start_pos;

    let mut visited_pos = HashSet::from([input.start_pos]);

    loop {
        if !in_bounds(&input.grid, (pos.0 + dir.0, pos.1 + dir.1)) {
            break;
        }

        if input.grid[(pos.0 + dir.0) as usize][(pos.1 + dir.1) as usize] == '#' {
            dir = (dir.1, -dir.0);
            continue;
        }

        pos = (pos.0 + dir.0, pos.1 + dir.1);

        visited_pos.insert(pos);
    }

    visited_pos.len() as isize
}
