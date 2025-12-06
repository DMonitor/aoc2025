use std::collections::HashSet;
use std::ops::RangeInclusive;


const INPUT0: &str = include_str!("../res/input0");
const INPUT1: &str = include_str!("../res/input1");

type RangeSet = HashSet<RangeInclusive<u64>>;

fn parse_file() -> (RangeSet, Vec<u64>)
{

    let ingredient_ranges = INPUT0.trim()
        .lines()
        .into_iter()
        .map( |line| {
            let range = line.split_at(line.find('-').unwrap());
            RangeInclusive::new(
                range.0.parse::<u64>().unwrap(),
                range.1[1..].parse::<u64>().unwrap()
            )
        }).collect::<RangeSet>();

    let test_ingredients = INPUT1
        .trim()
        .lines()
        .map(|s|
            s.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    (ingredient_ranges,test_ingredients)
}

fn solve_2() -> u32 {
    0
}

fn solve_1() -> u32 {

    let (good_ingredients, pantry) = parse_file();
    pantry
        .iter()
        .filter(|i|
            good_ingredients
                .iter()
                .any(|r| r.contains(i)))
        .count() as u32
}

fn main()
{
    println!("{}", solve_1());
    println!("{}", solve_2());
}