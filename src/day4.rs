use aoc_runner_derive::{aoc, aoc_generator};

type Line = Vec<char>;
type Grid = Vec<Line>;
type Int = usize;
type Direction = (isize, isize);

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

const DIRECTIONS: [Direction; 8] = [
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn is_in_bounds(grid: &Grid, i: isize, j: isize) -> bool {
    i >= 0 && i < grid.len() as isize && j >= 0 && j < grid[0].len() as isize
}

fn try_move(grid: &Grid, i: Int, j: Int, dir: Direction) -> Option<(Int, Int)> {
    match is_in_bounds(grid, i as isize + dir.0, j as isize + dir.1) {
        true => Some(((i as isize + dir.0) as usize, (j as isize + dir.1) as usize)),
        false => None,
    }
}

fn is_valid_xmas(grid: &Grid, mut i: Int, mut j: Int, dir: Direction) -> bool {
    for c in "MAS".chars() {
        match try_move(grid, i, j, dir) {
            Some((new_i, new_j)) => {
                if grid[new_i][new_j] != c {
                    return false;
                }
                i = new_i;
                j = new_j;
            }
            None => return false,
        }
    }
    true
}

fn count_valid_directions(grid: &Grid, i: Int, j: Int) -> Int {
    let mut res = 0;

    for dir in DIRECTIONS {
        if is_valid_xmas(grid, i, j, dir) {
            res += 1;
        }
    }

    res
}

#[aoc(day4, part1)]
fn part_one(grid: &Grid) -> Int {
    let mut res = 0;

    for (i, line) in grid.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == 'X' {
                res += count_valid_directions(grid, i, j)
            }
        }
    }

    res
}

fn is_valid_x_mas(grid: &Grid, i: Int, j: Int) -> bool {
    true
}

#[aoc(day4, part2)]
fn part_two(grid: &Grid) -> Int {
    let mut res = 0;

    for (i, line) in grid.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == 'A' && is_valid_x_mas(grid, i, j) {
                res += 1
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_one() {
        assert_eq!(part_one(&parse_input(INPUT)), 18)
    }

    #[test]
    fn test_two() {
        assert_eq!(part_two(&parse_input(INPUT)), 9)
    }
}
