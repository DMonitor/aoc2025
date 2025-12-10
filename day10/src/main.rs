use regex::{Regex, RegexBuilder};
const INPUT: &str = include_str!("../res/test");


struct ButtonSet {
    lights:Vec<bool>,
    buttons: Vec<Vec<u32>>,
    jolts: Vec<u32>,
}


fn parse_input(file:&str) -> (Vec<ButtonSet>)
{
    let line_parse = Regex::new(r"\[([.#]+)] ((?:\([0-9,]+\)\s?)+)\{([0-9,]+)}").unwrap();

    let s = file.lines().map(
        |line| {
            line_parse.captures(&line)
        }
    ).collect::<Vec<_>>();

    let mut result = Vec::<ButtonSet>::new();

    for line in s {

        let line = line.unwrap();

        let light_pattern = line.get(1).unwrap().as_str();

        let lights = light_pattern.chars().map(|c| match c {
            '.' => false,
            '#' => true,
            _ => unreachable!()
        }).collect::<Vec<_>>();

        let button_pattern = line.get(2).unwrap().as_str()
            .replace('(',"")
            .replace(')',"");

        let buttons = button_pattern.split_terminator(" ")
            .map(|s| s.split_terminator(',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>()).collect::<Vec<_>>();

        result.push(ButtonSet { lights, buttons, jolts:vec!()})
    }

    (result)


}

fn solve_2(input: &str) -> u32 {
    0
}

fn solve_1(input: &str) -> u32 {
    let button_sets = parse_input(input);

    for set in button_sets {
        println!("{:?}", set.lights);
        println!("{:?}", set.buttons);
    }



    0
}

fn main()
{


    println!("{}", solve_1(INPUT));
    println!("{}", solve_2(INPUT));
}