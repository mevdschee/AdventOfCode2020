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
            let first: usize = cap[1].parse().unwrap();
            let second: usize = cap[2].parse().unwrap();
            let chr: char = cap[3].parse().unwrap();
            let pwd: String = cap[4].parse().unwrap();
            let ch1 = pwd.chars().nth(first - 1).unwrap_or('!');
            let ch2 = pwd.chars().nth(second - 1).unwrap_or('!');
            let mut n = 0;
            if ch1 == chr {
                n += 1;
            }
            if ch2 == chr {
                n += 1;
            }
            if n == 1 {
                i += 1;
            }
        }
    }
    println!("{}", i);
}
