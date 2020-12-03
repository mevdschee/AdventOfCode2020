use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input").expect("cannot open file");
    let file = BufReader::new(file);
    let mut i = 0;
    for line in file.lines() {
        let s: String = line.ok().unwrap();
        let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
        for cap in re.captures_iter(&s) {
            let min: i32 = cap[1].parse().unwrap();
            let max: i32 = cap[2].parse().unwrap();
            let chr: char = cap[3].parse().unwrap();
            let pwd: String = cap[4].parse().unwrap();
            let mut n = 0;
            for c in pwd.chars() {
                if c == chr {
                    n += 1;
                }
            }
            if n >= min && n <= max {
                i += 1;
            }
        }
    }
    println!("{}", i);
}
