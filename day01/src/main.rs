use std::path::absolute;

const INPUT: &str = include_str!("../res/input");

fn parse_file(file: &str) -> Vec<i32> {
    file.lines()
        .map(|line| {
            let mut iter = line.chars();
            let direction = iter.next().unwrap();
            let number = iter.as_str().parse::<i32>().unwrap();

            match direction {
                'L' => -number,
                'R' => number,
                _ => panic!(),
            }
        })
        .collect()
}

fn solve_1() {
    let mut neutrals: u32 = 0;
    let mut position: i32 = 50;
    let inputs = parse_file(INPUT);

    for number in inputs {
        position += number;
        neutrals += if position % 100 == 0 { 1 } else { 0 };
    }

    println!("neutrals: {}", neutrals);
}

fn solve_2() {
    let mut clicks = 0;
    let mut position: i32 = 50;
    let inputs = parse_file(INPUT);
    for number in inputs {
        println!("rotate {}", number);
        // if greater than 100, those are inherent clicks
        println!("inherent {}", (number / 100).abs());
        clicks += (number / 100).abs();

        // keep the remainder
        println!("{} to {}", position, position + number % 100);

        // if we start at 0, we cannot cause non-inherent clicks
        if (position != 0) {
            position += number % 100;
            match position {
                100.. => {
                    println!("100 or more");
                    clicks += 1
                }
                ..=0 => {
                    println!("0 or less");
                    clicks += 1;
                }
                _ => println!("fine"),
            }
        } else {
            position += number % 100;
        }

        position = position % 100;
        if (position < 0) {
            position += 100;
        }
        println!("position is now {}", position);
    }

    println!("{} clicks", clicks)
}

fn main() {
    solve_2();
}
