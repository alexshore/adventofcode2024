fn main() {
    let input = include_str!("../input.txt").trim_end();
    println!("{}", part_one(input));
    println!("{}", part_two(input))
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut a = Vec::new();
    let mut b = Vec::new();

    input.lines().for_each(|line| {
        let mut split_line = line.split_whitespace();
        a.push(split_line.next().unwrap().parse::<u32>().unwrap());
        b.push(split_line.next().unwrap().parse::<u32>().unwrap());
    });

    (a, b)
}

fn part_one(input: &str) -> String {
    let (mut a, mut b) = parse_input(input);

    a.sort_unstable();
    b.sort_unstable();

    a.into_iter()
        .zip(b)
        .fold(0, |acc, t| acc + t.0.abs_diff(t.1))
        .to_string()
}

fn part_two(input: &str) -> String {
    let (mut a, b) = parse_input(input);

    a.sort_unstable();
    a.dedup();

    a.iter()
        .fold(0, |acc, a_val| {
            acc + (b.iter().filter(|&b_val| *b_val == *a_val).count() as u32 * *a_val)
        })
        .to_string()
}
