use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let file = File::open("input").expect("cannot open file");
    let file = BufReader::new(file);
    let mut or = 0;
    let mut and = 0;
    let mut mem = HashMap::<i64, i64>::new();
    for line in file.lines() {
        let s = line.unwrap();
        let re = Regex::new(r"(mask = ([X01]+)|mem\[([0-9]+)\] = ([0-9]+))").unwrap();
        let caps = re.captures(&s).unwrap();
        if caps.get(2).is_some() {
            let binary = caps.get(2).unwrap().as_str();
            or = i64::from_str_radix(&binary.replace("X", "0"), 2).unwrap();
            and = i64::from_str_radix(&binary.replace("X", "1"), 2).unwrap();
        } else {
            let pos: i64 = caps.get(3).unwrap().as_str().parse().unwrap();
            let val: i64 = caps.get(4).unwrap().as_str().parse().unwrap();
            let value = (val & and) | or;
            mem.insert(pos, value);
        }
    }
    let total: i64 = mem.values().sum();
    println!("{}", total);
}
