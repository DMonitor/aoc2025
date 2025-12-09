use std::collections::{HashSet};
use lib::points::Point3D;

const INPUT: &str = include_str!("../res/input");

fn parse_input(input:&str) -> HashSet<Point3D>
{
    input
        .lines()
        .map(|line|
            line.split_terminator(',').map(|x| x.parse::<i64>().unwrap()))
        .map(|mut points|
            Point3D {x: points.next().unwrap(), y: points.next().unwrap(), z: points.next().unwrap()})
        .collect()
}

fn solve_2(input: &str) -> u64 {
    let point_map = parse_input(input);

    let mut relations:Vec<(Point3D, Point3D, i64)> =
        point_map.iter().map( |Point3D|
            point_map
                .iter()
                .map(|p| (p,p.distance_cmp(*Point3D)))
                .filter(|p| p.1 != 0)
                .map(|p| (Point3D.clone(),*p.0,p.1)).collect::<Vec<(Point3D, Point3D, i64)>>())
            .flatten().collect();

    // sorted with closest at the bottom
    relations.sort_by(|a,b| b.2.cmp(&a.2));

    let goal = point_map.len();

    // each circuit having 2 cells is a fair assumption
    let mut circuits = Vec::<HashSet<Point3D>>::with_capacity(goal /2);

    let (mut x, mut y):(Point3D,Point3D) = (Point3D {x:0,y:0,z:0},Point3D {x:0,y:0,z:0});

    while !circuits.iter().any(|c| c.len() >= goal)
    {
        let close = relations.pop().unwrap();
        x = close.0;
        y = close.1;
        relations.pop();
        let mut neighbor_circuits = circuits.iter_mut().filter( |c| c.contains(&close.0) || c.contains(&close.1)).collect::<Vec<&mut HashSet<Point3D>>>();

        //println!("{} {}", close.0, close.1);
        match neighbor_circuits.len()
        {
            0 => {
                println!("new");
                // make a new circuit
                let mut new_circuit:HashSet<Point3D> = HashSet::new();
                new_circuit.insert(close.0);
                new_circuit.insert(close.1);
                circuits.push(new_circuit);
            }
            1 => {
                // add to circuit
                neighbor_circuits[0].insert(close.0);
                neighbor_circuits[0].insert(close.1);
            }
            2 => {
                println!("merge");
                // merge two circuits
                let (circuit1,circuit2) = neighbor_circuits.split_at_mut(1);
                circuit1[0].extend(circuit2[0].iter());
                circuit2[0].clear();
            }
            _ => unreachable!()
        }

    }

    (x.x * y.x) as u64

}

fn solve_1(input: &str) -> u64 {

    // apparently you don't need to check whether a box is in a circuit before adding it to the circuit
    // I could fix this solution to make it less nonsensical with that in mind, but I wont

    let point_map = parse_input(input);

    let mut relations:Vec<(Point3D, Point3D, i64)> = Vec::with_capacity(point_map.len());

    for Point3D in point_map.iter()
    {
        let me = Point3D.clone();
        let closest = point_map
                .iter()
                .map(|p| (p,p.distance_cmp(*Point3D)))
                .filter(|p| p.1 != 0)
                .map(|p| (me,*p.0,p.1));
        relations.extend(closest);
    }

    // sorted with closest at the bottom
    relations.sort_by(|a,b| b.2.cmp(&a.2));

    const LINKS:usize = 10;

    // each circuit having 2 cells is a fair assumption
    let mut circuits = Vec::<HashSet<Point3D>>::with_capacity(LINKS/2);
    let mut made_links = 0;

    while made_links < LINKS
    {
        let close = relations.pop().unwrap();
        relations.pop();
        let mut neighbor_circuits = circuits.iter_mut().filter( |c| c.contains(&close.0) || c.contains(&close.1)).collect::<Vec<&mut HashSet<Point3D>>>();

        println!("{} {}", close.0, close.1);
        match neighbor_circuits.len()
        {
            0 => {
                println!("new");
                // make a new circuit
                let mut new_circuit:HashSet<Point3D> = HashSet::new();
                new_circuit.insert(close.0);
                new_circuit.insert(close.1);
                circuits.push(new_circuit);
                made_links += 1;
            }
            1 => {
                // add to circuit
                neighbor_circuits[0].insert(close.0);
                neighbor_circuits[0].insert(close.1);
                made_links += 1;
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

    println!("total links: {}", made_links);
    total as u64
}

fn main()
{
    //println!("{}", solve_1(INPUT));
    println!("{}", solve_2(INPUT));
}