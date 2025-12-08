
const INPUT: &str = include_str!("../res/input");

enum PointType {
    START,
    EMPTY,
    SPLITTER
}

// each item in the vec is a row. Each item in a row is a point
type Map = Vec<Vec<PointType>>;

fn parse_input(file:&str) -> Map {
    file.lines().map(|l| l.chars().map(|c| match c {
        'S' => PointType::START,
        '.' => PointType::EMPTY,
        '^' => PointType::SPLITTER,
        _ => unreachable!()
    }).collect()).collect()
}

fn print_map(line:&Vec<PointType>, state:&Vec<u64>) {
    for (loc,point) in line.iter().enumerate() {
        match point {
            PointType::START => print!("S"),
            PointType::EMPTY => print!("{}", if state[loc] == 1 {'|'} else {'.'} ),
            PointType::SPLITTER => print!("^")
        }
    }
    println!();
}
fn print_map_quantum(line:&Vec<PointType>, state:&Vec<u64>) {
    for (loc,point) in line.iter().enumerate() {
        match point {
            PointType::START => print!("S"),
            PointType::EMPTY => print!("{}", if state[loc] >= 1 {state[loc]} else {0} ),
            PointType::SPLITTER => print!("^")
        }
    }
    println!();
}


fn solve_2(input: &str) -> u64 {
    let map = parse_input(input);
    let width = map[0].len();

    let mut state = vec!(0;width);

    for line in &map {
        for (loc,point) in line.iter().enumerate() {
            match point {
                PointType::START => state[loc] = 1,
                PointType::EMPTY => continue,
                PointType::SPLITTER => {
                    if state[loc] != 0 {
                        state[loc - 1] += state[loc];
                        state[loc + 1] += state[loc];
                        state[loc] = 0;
                    }
                }
            }
        }
        //print_map_quantum(&line, &state);
    }

    state.iter().sum()

}

fn solve_1(input: &str) -> u64 {
    let map = parse_input(input);
    let width = map[0].len();

    let mut state = vec!(0;width);

    let mut splits = 0;

    for line in &map {
        for (loc,point) in line.iter().enumerate() {
            match point {
                PointType::START => state[loc] = 1,
                PointType::EMPTY => continue,
                PointType::SPLITTER => {
                    if state[loc] != 0 {
                        splits += 1;
                        state[loc] = 0;
                        state[loc - 1] = 1;
                        state[loc + 1] = 1;
                    }
                }
            }
        }
        print_map(&line, &state);
    }

    splits

}

fn main()
{


    println!("{}", solve_1(INPUT));
    println!("{}", solve_2(INPUT));
}