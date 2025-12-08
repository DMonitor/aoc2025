mod points;

use std::collections::{HashMap, HashSet};
use points::Point;

const INPUT: &str = include_str!("../res/test");

fn parse_input(input:&str) -> HashSet<Point>
{
    input
        .lines()
        .map(|line|
            line.split_terminator(',').map(|x| x.parse::<i64>().unwrap()))
        .map(|mut points|
            Point {x: points.next().unwrap(), y: points.next().unwrap(), z: points.next().unwrap()})
        .collect()
}

fn solve_2(input: &str) -> u64 {
    0
}

fn solve_1(input: &str) -> u64 {

    let point_map = parse_input(input);

    let mut relations:Vec<(Point, (&Point, f64))> = Vec::with_capacity(point_map.len());

    for point in point_map.iter()
    {
        let me = point.clone();
        let closest = point_map
                .iter()
                .map(|p| (p,p.distance(*point)))
                .filter(|p| p.1 != 0f64)
                .map(|p| (me,(p.0,p.1)));
        relations.extend(closest);
    }

    // sorted with closest at the bottom
    relations.sort_by(|a,b| b.1.1.partial_cmp(&a.1.1).unwrap());

    const LINKS:usize = 10;

    // each circuit having 2 cells is a fair assumption
    let mut circuits = Vec::<HashSet<Point>>::with_capacity(LINKS/2);

    let mut iterator = 0;

    let mut made_links = 0;
    while made_links < LINKS
    {
        let close = relations.pop().unwrap();
        println!("{} {}", close.0, close.1.0);

        let mut neighbor_circuits = circuits.iter_mut().filter( |c| c.contains(&close.0) || c.contains(&close.1.0)).collect::<Vec<&mut HashSet<Point>>>();

        match neighbor_circuits.len()
        {
            0 => {
                println!("new");
                // make a new circuit
                let mut new_circuit:HashSet<Point> = HashSet::new();
                new_circuit.insert(close.0);
                new_circuit.insert(*close.1.0);
                circuits.push(new_circuit);
                made_links += 1;
            }
            1 => {
                println!("add to {:?}",neighbor_circuits[0]);
                // add to circuit (just try both)
                // need to bypass short circuit logic
                let i1=  neighbor_circuits[0].insert(close.0);
                let i2 = neighbor_circuits[0].insert(*close.1.0);
                if i1 || i2 {
                    made_links += 1;
                }
            }
            2 => {
                println!("merge");
                // merge two circuits
                let (circuit1,circuit2) = neighbor_circuits.split_at_mut(1);
                circuit1[0].extend(circuit2[0].iter());
                circuit2[0].clear();
                made_links += 1;
            }
            _ => unreachable!()
        }

    }

    println!();
    for c in circuits.iter()
    {

        for p in c.iter() {
            print!("{} ",p);
        }
        println!();
    }

    let mut x = circuits.iter().map(|c| c.len()).collect::<Vec<usize>>();
    println!("{:?}", x);
    x.sort();
    println!("{:?}", x);
    let mut total = 1;
    for c in x.iter().rev().take(3)
    {
        println!("{}",c);
        total *= c;
    }

    total as u64
}

fn main()
{
    println!("{}", solve_1(INPUT));
    println!("{}", solve_2(INPUT));
}