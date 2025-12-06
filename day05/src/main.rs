use std::collections::HashSet;
use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../res/input");



fn parse_file(input :&str) -> (HashSet<u64>, Vec<u64>)
{
    let (ranges,values) = input.split_at(input.find("\r\n\r\n").unwrap());

    let ingredient_ranges = ranges
        .lines()
        .into_iter()
        .map( |line| {
            let range = line.split_at(line.find('-').unwrap());
            RangeInclusive::new(range.0.parse::<u64>().unwrap(),range.1[1..].parse::<u64>().unwrap())
        })
        .flatten();

    let test_ingredients = values.trim().lines().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    (HashSet::from_iter(ingredient_ranges),test_ingredients)
}

fn solve_2(input: &str) -> u32 {
    0
}

fn solve_1(input: &str) -> u32 {
    let (good_ingredients, pantry) = parse_file(input);

    pantry.iter().filter(|i| good_ingredients.contains(i)).count() as u32

}

fn main()
{
    println!("{}", solve_1(INPUT));
    println!("{}", solve_2(INPUT));
}