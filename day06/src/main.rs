use std::str::FromStr;

const INPUT: &str = include_str!("../res/test");

trait Evaluate {
    fn mul(&self) -> u64;
}

impl Evaluate for Vec<u64> {
    fn mul(&self) -> u64 {
        self.iter().fold(1, |acc, x| acc * x)
    }
}

enum Operator
{
    MULTIPLY,
    ADD
}

fn parse_file_1(file: &str) -> (Vec<u64>, Vec<Operator>) {

    let as_lines = file.lines().collect::<Vec<_>>();

    let n = as_lines
        .split_last()
        .unwrap().1
        .iter()
        .map(
            |x| x
                .split_whitespace()
                .map(|s| u64::from_str(s).unwrap()))
        .flatten().collect::<Vec<_>>();

    let o = as_lines
        .last()
        .unwrap()
        .split_whitespace()
        .map(|x| match x {
        "+" => Operator::ADD,
        "*" => Operator::MULTIPLY,
        _ => unreachable!()
    }).collect::<Vec<_>>();

    (n, o)

}



fn solve_1(input: &str) -> u64 {

    let (numbers, ops) = parse_file_1(INPUT);


    let mut total:u64 = 0;
    for (col,op) in ops.iter().enumerate() {

        let n = numbers[col..].iter().cloned().step_by(ops.len()).collect::<Vec<u64>>();
        let res = match op {
            Operator::MULTIPLY => n.mul(),
            Operator::ADD => n.iter().sum(),
        };
        total += res;

    }
    total
}

fn main() {
    println!("{}", solve_1(INPUT));
}