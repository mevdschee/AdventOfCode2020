use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input").expect("cannot open file");
    let file = BufReader::new(file);
    let mut i = 0;
    let mut x = 0;
    let mut len = 0;
    for line in file.lines() {
        if len == 0 {
            len = line.unwrap().chars().count();
        } else {
            x = (x + 3) % len;
            if line.unwrap().chars().nth(x).unwrap() == '#' {
                i += 1;
            }
        }
    }
    println!("{}", i);
}
