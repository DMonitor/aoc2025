use lib::points::{Point2D};

const INPUT: &str = include_str!("../res/input");

fn parse_file(input: &str) -> Vec<Point2D> {
    input
        .lines()
        .map(|line| {
            let l = line.split_once(',').unwrap();
            Point2D {x: l.0.parse().unwrap(), y: l.1.parse().unwrap()}})
        .collect()
}

fn solve_2(input: &str) -> u64 {

    let points = parse_file(input);

    0
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
    println!("{}", solve_1(INPUT));
    println!("{}", solve_2(INPUT));
}