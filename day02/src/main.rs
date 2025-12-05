use std::collections::HashSet;

const INPUT: &str = include_str!("../res/input");

fn parse_file(file: &str) -> Vec<(u64, u64)> {
    file.split_terminator(',')
        .map(|s| {
            let idk = s
                .split_terminator('-')
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            (idk[0], idk[1])
        })
        .collect()
}

fn solve1() {
    let ranges = parse_file(INPUT);

    let mut sum = 0;

    for pair in ranges {
        println!("{:?}", pair);
        for code in pair.0..=pair.1 {
            // codes have even set of digits
            let digits = code.ilog10();

            if digits % 2 == 1 {
                let upper = code / u64::pow(10, digits / 2 + 1);
                if (code - upper) % u64::pow(10, digits / 2 + 1) == 0 {
                    println!("{} is numberwang", code);
                    sum += code;
                }
            }
        }
    }
    println!("{}", sum);
}

fn solve2() {
    let mut seen: HashSet<u64> = HashSet::new();
    let ranges = parse_file(INPUT);
    let mut sum = 0;
    for pair in ranges {
        for code in pair.0..=pair.1 {
            if seen.contains(&code) {
                continue;
            }
            sum += get_numberwang(code);
            seen.insert(code);
        }
    }
    println!();
    println!("{}", sum);
}

fn get_numberwang(number: u64) -> u64 {
    let number_str = number.to_string();

    // check for the first character repeated
    if number > 9 && number_str.eq(&number_str[..1].repeat(number_str.len())) {
        println!("{}", number);
        return number;
    }

    // check for groupings
    for div in divisors::get_divisors(number_str.len()) {
        // division length
        let l = number_str.len() / div;
        if number_str.eq(&number_str[0..l].repeat(div)) {
            println!("{}", number);
            return number;
        }
    }

    // return 0
    0
}

fn main() {
    solve2();
}
