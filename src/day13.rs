use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

type Int = isize;

#[derive(Debug)]
struct Equation {
    a: Int,
    b: Int,
    goal: Int,
}

#[derive(Debug)]
struct Machine {
    x_eq: Equation,
    y_eq: Equation,
}

impl Machine {
    fn solve(&self) -> Option<Int> {
        let kx_iy = self.y_eq.a * self.x_eq.goal - self.x_eq.a * self.y_eq.goal;
        let kj_il = self.y_eq.a * self.x_eq.b - self.y_eq.b * self.x_eq.a;

        let b = if kx_iy % kj_il != 0 {
            return None;
        } else {
            kx_iy / kj_il
        };

        let y_lb = self.y_eq.goal - self.y_eq.b * b;

        let a = if y_lb % self.y_eq.a != 0 {
            return None;
        } else {
            y_lb / self.y_eq.a
        };

        Some(a * 3 + b)
    }
}

#[aoc_generator(day13)]
fn parse_input(input: &str) -> Vec<Machine> {
    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();

    input
        .split("\n\n")
        .map(|machine| {
            let caps = re.captures(machine).unwrap();

            Machine {
                x_eq: Equation {
                    a: caps[1].parse().unwrap(),
                    b: caps[3].parse().unwrap(),
                    goal: caps[5].parse().unwrap(),
                },
                y_eq: Equation {
                    a: caps[2].parse().unwrap(),
                    b: caps[4].parse().unwrap(),
                    goal: caps[6].parse().unwrap(),
                },
            }
        })
        .collect()
}

#[aoc(day13, part1)]
fn part_one(input: &Vec<Machine>) -> Int {
    input
        .iter()
        .map(|machine| match machine.solve() {
            Some(x) => x,
            None => 0,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&parse_input(INPUT)), 480)
    }
}
