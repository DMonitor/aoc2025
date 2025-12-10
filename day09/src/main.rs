use lib::points::{Point2D};
use lib::space::{Space};

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

    let (compressed_points,point_map) = Point2D::compress2d(points.clone());

    // we can probably skip this step by iterating through points and doing the point map lookup later on in the math
    //let compressed_points = points.iter().map( |p| *point_map.get(p).unwrap()).collect::<Vec<_>>();
    //let compressed_points = points;


    let x_max = compressed_points.iter().map(|p| p.x).max().unwrap();
    let y_max = compressed_points.iter().map(|p| p.x).max().unwrap();

    let mut pairs = compressed_points.iter().zip(compressed_points.iter().skip(1)).collect::<Vec<(&Point2D, &Point2D)>>();
    pairs.push((compressed_points.last().unwrap(),compressed_points.first().unwrap()));

    let mut space = Space::<char>::new(x_max as u64 + 1,y_max as u64 + 1, '.');

    for pair in pairs {
        space.draw_line_point(pair.0, pair.1, '#');
    }

    let mut midpoint = None;


    // ray cast to find middle
    for y in 0..y_max as u64 {
        let mut state = 0;
        for x in 0..x_max as u64 {
            match state {
                0 => {
                    if space.get(x,y).unwrap() == '#' {
                        state = 1;
                    }
                }
                1 => {
                    if space.get(x,y).unwrap() == '.' {
                        midpoint = Some(Point2D { x: x as i64, y: y as i64});
                        //println!("midpoint: {:?}",midpoint);
                        break;
                    }
                    else {
                        break;
                    }
                }
                _ => unreachable!()
            }
        }
        if midpoint.is_some()
        {
            break;
        }
    }

    space.flood_fill_point(&midpoint.unwrap(), '.', 'X');

    for p in compressed_points.iter() {
        space.set_point(p, '@');
    }

    // find rectangles like in part 1, except use the any_inside function
    let mut areas = get_areas_p2(compressed_points.iter().collect());
    areas.sort_by(|a,b| b.0.cmp(&a.0));

    let winner = areas.iter().find(|(_, p1, p2)| !space.any_inside_points('.',p1,p2).unwrap());
    if winner.is_none() {
        println!("winner not found");
        return 0;
    }
    let winner = winner.unwrap();

    //println!("winner: {:?}",winner);

    //println!("{}",space.get_rectangle_points(winner.1,winner.2).unwrap());
    let real_winner_1 = point_map.get(winner.1).unwrap();
    let real_winner_2 = point_map.get(winner.2).unwrap();
    //println!("winner: ({},{})",real_winner_1,real_winner_2);
    let real_area = real_winner_1.area_rectangle(&real_winner_2);
    // 1343576598
    // 186362000 is too low
    real_area
}

fn get_areas(points1:Vec<&Point2D>, points2:Vec<&Point2D>) -> u64
{
    points1.iter().map(|p| {
        points2.iter().map(|p2| p.area_rectangle(&p2)).max().unwrap_or(0)
    }).max().unwrap_or(0)
}

fn get_areas_p2(points:Vec<&Point2D>) -> Vec<(u64,&Point2D, &Point2D)>
{
    points.iter().map(|p1| {
        points.iter().map(|p2| (p1.area_rectangle(&p2), *p1, *p2))
    }).flatten().collect::<Vec<_>>()
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