use regex::{Regex, RegexBuilder};
use itertools::Itertools;

const INPUT: &str = include_str!("../res/input");


struct ButtonSet {
    lights:u32,
    buttons: Vec<Vec<u32>>,
    jolts: Vec<u32>,
}


fn parse_input(file:&str) -> Vec<ButtonSet>
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

        let lights = light_pattern.chars().enumerate().fold(0u32,|acc, (i,c)| match c {
            '.' => acc,
            '#' => acc | (1 << i),
            _ => unreachable!()
        });

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


    let mut answer = 0;
    // get all the combinations of button presses, sort by smallest, find first that results in pattern
    for set in button_sets {
        let final_lights = set.lights;
        // make each button a bit mask
        let masks = set.buttons.iter().map(|b| {
            b.iter().fold(0u32, |acc,b| acc | (1u32 << b) )
        }).collect::<Vec<_>>();
        let mut solution:Option<Vec<&u32>> = None;
        for i in 1..=masks.len() {
            let c = masks.iter().combinations(i);
            for a in c {
                let mut current_lights = 0u32;
                for press in a.iter() {
                    current_lights = current_lights ^ **press;
                }
                if current_lights == final_lights {
                    solution = Some(a);
                    break;
                }
            }
            if solution.is_some() {
                break;
            }
        }
        let solution = solution.unwrap();
        println!("solution: {:?} {}", solution, solution.len());
        answer += solution.len() as u32;
    }



    answer
}

fn main()
{


    println!("{}", solve_1(INPUT));
    println!("{}", solve_2(INPUT));
}