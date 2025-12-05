const INPUT: &str = include_str!("../res/input");

fn parse_file(file: &str) -> Vec<Vec<u8>> {
    file.lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|&b| if b == b'@' { 1 } else { 0 })
                .collect::<Vec<u8>>()
        })
        .collect()
}

fn check_index(x: usize, y: usize, matrix: &Vec<Vec<u8>>) -> bool {
    let x_low = if x == 0 { 0 } else { x - 1 };
    let x_high = if x == matrix[0].len() - 1 {
        matrix[0].len() - 1
    } else {
        x + 1
    };

    let y_low = if y == 0 { 0 } else { y - 1 };
    let y_high = usize::min(matrix.len() - 1, y + 1);

    matrix[y_low..=y_high]
        .iter()
        .map(|row| row[x_low..=x_high].iter().map(|&b| b as u32).sum::<u32>())
        .sum::<u32>()
        < 5
}

fn solve1() {
    let mut total = 0;
    let lines = parse_file(INPUT);
    let height = lines.len();
    let width = lines[0].len();

    for y in 0..height {
        for x in 0..width {
            if (lines[y][x] == 1) {
                print!(
                    "{}",
                    if check_index(x, y, &lines) {
                        total += 1;
                        'x'
                    } else {
                        '@'
                    }
                );
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!("Total: {}", total);
}

fn cleanup(matrix: &mut Vec<Vec<u8>>) -> u32 {
    let new_matrix = matrix.clone();
    let mut total = 0;
    let height = new_matrix.len();
    let width = new_matrix[0].len();

    for y in 0..height {
        for x in 0..width {
            if (new_matrix[y][x] == 1) && check_index(x, y, &new_matrix) {
                matrix[y][x] = 0;
                total += 1;
            }
        }
    }
    total
}

fn solve2() {
    let mut total = 0;
    let mut lines = parse_file(INPUT);

    loop {
        let r = cleanup(&mut lines);
        if r == 0 {
            break;
        }
        println!("removed {}", r);
        total += r;
    }

    println!("Total: {}", total);
}

fn main() {
    solve2();
}
