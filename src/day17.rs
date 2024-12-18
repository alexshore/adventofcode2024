use aoc_runner_derive::{aoc, aoc_generator};

type Int = u64;

#[derive(Clone, Debug)]
struct CPU {
    program: Vec<Int>,
    program_str: String,
    reg_a: Int,
    reg_b: Int,
    reg_c: Int,
    pc: usize,
}

impl CPU {
    fn new(program: Vec<Int>, program_str: String, reg_a: Int, reg_b: Int, reg_c: Int) -> Self {
        Self {
            program,
            program_str,
            reg_a,
            reg_b,
            reg_c,
            pc: 0,
        }
    }

    fn get_combo(&self, operand: Int) -> Int {
        match operand {
            0..=3 => operand,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => unreachable!(),
        }
    }

    fn run_program(&mut self) -> String {
        let mut res = String::new();

        loop {
            if self.pc >= self.program.len() {
                break;
            }

            match self.program[self.pc] {
                0 => self.reg_a = self.reg_a / (1 << self.get_combo(self.program[self.pc + 1])),
                1 => self.reg_b = self.reg_b ^ self.program[self.pc + 1],
                2 => self.reg_b = self.get_combo(self.program[self.pc + 1]) % 8,
                3 => {
                    if self.reg_a != 0 {
                        self.pc = self.program[self.pc + 1] as usize;
                        // we continue so pc doesn't get incremented post instruction
                        continue;
                    }
                }
                4 => self.reg_b = self.reg_b ^ self.reg_c,
                5 => {
                    if !res.is_empty() {
                        res.push(',');
                    }

                    let push = (self.get_combo(self.program[self.pc + 1]) % 8).to_string();
                    res.push_str(&push);
                }
                6 => self.reg_b = self.reg_a / (1 << self.get_combo(self.program[self.pc + 1])),
                7 => self.reg_c = self.reg_a / (1 << self.get_combo(self.program[self.pc + 1])),
                _ => unreachable!(),
            }

            self.pc += 2;
        }

        res
    }
}

#[aoc_generator(day17)]
fn parse_input(input: &str) -> CPU {
    //     let input = "Register A: 2024
    // Register B: 0
    // Register C: 0

    // Program: 0,3,5,4,3,0";
    let (registers_str, program_str) = input.split_once("\n\n").unwrap();

    let mut registers_str_lines = registers_str.lines();

    let reg_a = registers_str_lines
        .next()
        .unwrap()
        .rsplit_once(' ')
        .unwrap()
        .1
        .parse()
        .expect("failed to parse");

    let reg_b = registers_str_lines
        .next()
        .unwrap()
        .rsplit_once(' ')
        .unwrap()
        .1
        .parse()
        .expect("failed to parse");

    let reg_c = registers_str_lines
        .next()
        .unwrap()
        .rsplit_once(' ')
        .unwrap()
        .1
        .parse()
        .expect("failed to parse");

    let program = program_str
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap() as Int)
        .collect();

    CPU::new(
        program,
        program_str.rsplit_once(' ').unwrap().1.to_string(),
        reg_a,
        reg_b,
        reg_c,
    )
}

#[aoc(day17, part1)]
fn part_one(cpu: &CPU) -> String {
    println!("cpu: {cpu:?}");
    let mut cpu = cpu.to_owned();

    cpu.run_program()
}

#[aoc(day17, part2)]
fn part_two(cpu: &CPU) -> Int {
    let mut cpu = cpu.to_owned();
    let mut test_a = 0;
    let init_b = cpu.reg_b;
    let init_c = cpu.reg_c;

    loop {
        cpu.reg_a = test_a;
        cpu.reg_b = init_b;
        cpu.reg_c = init_c;
        cpu.pc = 0;

        let res = cpu.run_program();

        if cpu.program_str == res {
            return test_a;
        }

        if cpu.program_str.ends_with(res.as_str()) {
            test_a *= 8;
        } else {
            test_a += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const INPUT2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(&parse_input(INPUT1)),
            String::from("4,6,3,5,6,3,5,2,1,0")
        )
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&parse_input(INPUT2)), 117440)
    }
}
