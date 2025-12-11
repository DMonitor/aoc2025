use regex::Regex;
use itertools::Itertools;
use z3;
use z3::ast::Int;
use z3::ast::Bool;

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


        let jolts_pattern = line.get(3).unwrap().as_str();
        let jolts = jolts_pattern.split_terminator(',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
        result.push(ButtonSet { lights, buttons, jolts})
    }

    result


}

fn solve_2(input: &str) -> u64 {
    let button_sets = parse_input(input);
    let mut answer = 0;

    for set in button_sets {

        // track how many times each button has been pressed
        let presses:Vec<Int> = (0..set.buttons.len()).map(|i|
            Int::new_const(format!("button{}", i).as_str())).collect();

        // track total number of presses
        let press_total =  presses.iter().fold(Int::from_i64(0), |i,p| i + p);
        let mut jolts_resolved:Vec<Bool> = vec!();


        for light in 0..set.jolts.len() {
            let mut b:Vec<Int> = vec!();
            for (button_index,button) in set.buttons.iter().enumerate()
            {
                // if a button effects a light, track it
                if button.contains(&(light as u32)) {
                    b.push(presses[button_index].clone());
                }
            }

            // the total jolts equals the number of times relevant buttons have been pressed
            let j_total =  b.iter().fold(Int::from_i64(0), |i,b| i + b);
            jolts_resolved.push(j_total.eq(set.jolts[light]));
        }


        let solver = z3::Optimize::new();


        jolts_resolved.iter().for_each(|j| solver.assert(j));

        // negative solutions not allowed
        presses.iter().for_each(|p| solver.assert(&p.ge(0)));


        solver.minimize(&press_total);

        let b = Bool::new_const(".");
        solver.check(&[b]);
        let model = solver.get_model().unwrap();

        for button in presses
        {
            answer += model.get_const_interp(&button).unwrap().as_u64().unwrap();
        }
    }

    answer
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
        //println!("solution: {:?} {}", solution, solution.len());
        answer += solution.len() as u32;
    }

    answer
}

fn main()
{


    println!("{}", solve_1(INPUT));
    println!("{}", solve_2(INPUT));
}