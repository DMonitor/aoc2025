
const INPUT: &str = include_str!("../res/input");

fn parse_file(file: &str) -> Vec<&str> {
    file.lines().collect()
}

fn solve(size: usize) {
    let mut total: u64 = 0;
    for line in parse_file(INPUT).iter().map(|s| s.as_bytes()) {
        let mut this_jolts = 0;
        let mut last_index = 0;
        let line_len = line.len();
        for index in 1..=size {
            // our window is the index of last find & line length minus the remaining number of digits
            let j = line[last_index..line_len - size + index]
                .iter()
                .enumerate() // enumerate to get the index
                .rev() // reverse so we find the first one
                .max_by_key(|&(_idx, &val)| val) // find the max value in our set
                .unwrap();
            // update the window
            last_index = last_index + j.0 + 1;
            // once we find our jolt, add it to total
            this_jolts += (*j.1 as i64 - 48) * (i64::pow(10, (size - index) as u32));
        }
        println!("+{}", this_jolts);
        total += this_jolts as u64;
    }
    println!("={}", total);
}

fn main() {
    solve(12);
}
