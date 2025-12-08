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

fn solve_2(file: &str) -> u64 {

    let numbers = file
        .lines()
        .collect::<Vec<_>>()
        .split_last()
        .unwrap().1
        .iter()
        .map(|s| s
            .replace(" ", "0")
            .chars()
            .rev()
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut line_size = file
        .lines()
        .last()
        .unwrap()
        .split_inclusive(|c| c == '+' || c == '*')
        .map(|s| s.len()-1)
        .skip(1)
        .collect::<Vec<_>>();

    let ops_len = line_size.len()-1;
    line_size[ops_len] = line_size[line_size.len()-1] + 2;

    let ops =
    line_size.iter().zip(file
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .map(|x| match x {
            "+" => Operator::ADD,
            "*" => Operator::MULTIPLY,
            _ => unreachable!()
        }).collect::<Vec<_>>()).rev();


    let mut total = 0;
    let mut acc = 0;
    for (len,op) in ops {
        let s = numbers.iter().map( |s| &s[acc..*len+acc]).collect::<Vec<_>>();
        /*
            s contains an array holding one char arr for each line, which is the current problem
            start with the most significant figure, end with least

            ie for

            64
            23
            314

            the array contains
            [ [0, 4, 6], [0, 3, 2], [4, 1, 3] ]
         */

        let mut numbers:Vec<u64> = vec!(0;*len);
        let mut numbers_tens:Vec<u32> = vec!(s.len() as u32;*len);
        for l in s {
            for n in l.iter().enumerate().filter(|p| p.1 != &'0') {
                numbers_tens[n.0] -= 1;
                numbers[n.0] += (*n.1 as u64 - 48) * 10_u64.pow(numbers_tens[n.0]);
            }
        }
        acc += len+1;

        let fixed = numbers.iter().zip(numbers_tens.iter()).map(
            |(x,t)| x / 10_u64.pow(*t)).collect::<Vec<u64>>();

        let res = match op {
            Operator::MULTIPLY => fixed.mul(),
            Operator::ADD => fixed.iter().sum(),
        };

        total += res;

    }


    total

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
    //println!("{}", solve_1(INPUT));
    println!("{}", solve_2(INPUT));
}