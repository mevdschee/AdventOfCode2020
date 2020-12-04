use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let steps = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut n: i64 = 1;
    for step in steps.iter() {
        let (dx, dy) = step;
        let file = File::open("input").expect("cannot open file");
        let file = BufReader::new(file);
        let mut i = 0;
        let mut x = 0;
        let mut len = 0;
        for (y, line) in file.lines().enumerate() {
            if len == 0 {
                len = line.unwrap().chars().count();
            } else {
                if y % dy == 0 {
                    x = (x + dx) % len;
                    if line.unwrap().chars().nth(x).unwrap() == '#' {
                        i += 1;
                    }
                }
            }
        }
        n *= i;
    }
    println!("{}", n);
}
