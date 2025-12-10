use std::collections::{HashMap};
use lib::points::{Point2D};
use lib::space::{Space};

use crate::Direction::{Ascending, Descending};

const INPUT: &str = include_str!("../res/input");

fn parse_file(input: &str) -> Vec<Point2D> {
    input
        .lines()
        .map(|line| {
            let l = line.split_once(',').unwrap();
            Point2D {x: l.0.parse().unwrap(), y: l.1.parse().unwrap()}})
        .collect()
}

#[derive(PartialEq)]
enum Direction {
    Ascending,
    Descending
}


fn get_direction(c1:i64, c2:i64) -> Direction
{
    if c1 > c2 {
        return Ascending
    }
    Descending
}

fn compress_space(input_vec: &[i64]) -> HashMap<i64,usize>
{
    let mut workspace = input_vec.to_vec();
    workspace.sort();
    workspace.dedup();
    workspace.iter().enumerate().map(|(i,x)| (*x,i) ).collect()
}

fn solve_2(input: &str) -> u64 {

    let points = parse_file(input);

    let point_map = Point2D::compress2d(points.clone());

    // we can probably skip this step by iterating through points and doing the point map lookup later on in the math
    let compressed_points = points.iter().map( |p| *point_map.get(p).unwrap()).collect::<Vec<_>>();

    let x_max = compressed_points.iter().map(|p| p.x).max().unwrap();
    let y_max = compressed_points.iter().map(|p| p.x).max().unwrap();

    /*
    for p in compressed_points.iter().enumerate() {
        println!("{:?}", p);
    }
     */


    let mut pairs = compressed_points.iter().zip(compressed_points.iter().skip(1)).collect::<Vec<(&Point2D, &Point2D)>>();
    pairs.push((compressed_points.first().unwrap(), compressed_points.last().unwrap()));

    let mut space = Space::<bool>::new(x_max as u64 + 1,y_max as u64 + 1, false);



    for pair in pairs {
        space.draw_line_point(*pair.0, *pair.1, true);
    }

    println!("{}", space);
    // 186362000 is too low
    0
}


fn test_space(input: &str)
{
    let points = parse_file(input);

    let x_max = points.iter().map(|p| p.x).max().unwrap();
    let y_max = points.iter().map(|p| p.y).max().unwrap();

    let mut grid = Space::<bool>::new(14, 9, false);

    for point in points.iter() {
        grid.set_point(*point, true);
    }



    let mut pairs = points.iter().zip(points.iter().skip(1)).collect::<Vec<(&Point2D, &Point2D)>>();
    pairs.push((points.first().unwrap(), points.last().unwrap()));

    for pair in pairs {
        grid.draw_line_point(*pair.0, *pair.1, true);
    }
    println!("{}", grid);

    grid.flood_fill(3,4, true);

    println!("{}", grid);

}

fn get_max_area(point1:&Point2D, points:Vec<&Point2D>) -> u64
{
    points.iter().map(|p2| point1.area_rectangle(&p2)).max().unwrap_or(0)

}

fn get_areas(points1:Vec<&Point2D>, points2:Vec<&Point2D>) -> u64
{
    points1.iter().map(|p| {
        points2.iter().map(|p2| p.area_rectangle(&p2)).max().unwrap_or(0)
    }).max().unwrap_or(0)
}

fn solve_1(input: &str) -> u64 {
    let points = parse_file(input);

    let x_min = 0;
    let y_min = 0;
    let x_max = points.iter().map(|p| p.x).max().unwrap();
    let y_max = points.iter().map(|p| p.y).max().unwrap();

    // find midpoint of graph
    let x_mid = (x_min + x_max) / 2;
    let y_mid = (y_min + y_max) / 2;

    // divide graph into quads
    let quad0 = points.iter().filter(|p| p.x < x_mid && p.y <= y_mid).collect::<Vec<_>>();
    let quad1 = points.iter().filter(|p| p.x > x_mid && p.y < y_mid).collect::<Vec<_>>();
    let quad2 = points.iter().filter(|p| p.x < x_mid && p.y >= y_mid).collect::<Vec<_>>();
    let quad3 = points.iter().filter(|p| p.x > x_mid && p.y > y_mid).collect::<Vec<_>>();

    /*
    println!("quad0 {:?}", quad0);
    println!("quad1 {:?}", quad1);
    println!("quad2 {:?}", quad2);
    println!("quad3 {:?}", quad3);
    */

    // calculate areas only comparing points in opposing quadrants
    u64::max(
        get_areas(quad0,quad3),
        get_areas(quad1,quad2)
    )
}

fn main()
{
    //test_space(INPUT);
    //println!("{}", solve_1(INPUT));
    println!("{}", solve_2(INPUT));
}