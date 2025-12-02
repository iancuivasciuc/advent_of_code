use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const LOCK_SIZE: i32 = 100;
const START_POSITION: i32 = 50;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_rotation(s: &str) -> Option<(char, i32)> {
    if let Some(num_str) = s.strip_prefix('L') {
        num_str.parse().ok().map(|n| ('L', n))
    } else if let Some(num_str) = s.strip_prefix('R') {
        num_str.parse().ok().map(|n| ('R', n))
    } else {
        None
    }
}

fn main() -> io::Result<()> {
    let mut pos = START_POSITION;
    let mut zero_count = 0;

    let lines = read_lines("1.in")?;    

    for line in lines.map_while(Result::ok) {
        if let Some((dir, steps)) = parse_rotation(&line) {
            pos = match dir {
                'L' => (pos - steps + LOCK_SIZE) % LOCK_SIZE,
                'R' => (pos + steps) % LOCK_SIZE,
                _ => pos
            };

            if pos == 0 {
                zero_count += 1;
            }
        }
    }

    println!("Answer: {}", zero_count);
    Ok(())
}
