use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let file = File::open("input").expect("cannot open file");
    let file = BufReader::new(file);
    let mut ors = vec![];
    let mut mem = HashMap::<i64, i64>::new();
    for line in file.lines() {
        let s = line.unwrap();
        let re = Regex::new(r"(mask = ([X01]+)|mem\[([0-9]+)\] = ([0-9]+))").unwrap();
        let caps = re.captures(&s).unwrap();
        if caps.get(2).is_some() {
            let b = caps.get(2).unwrap().as_str();
            let xs = b.matches("X").count();
            ors = vec![];
            for i in 0..(1 << xs) {
                let mut orstr = b.to_string();
                let mut andstr = b.replace("0", "1");
                for x in 0..xs {
                    let c = match 1 << x & i > 0 {
                        true => "1",
                        false => "0",
                    };
                    orstr = orstr.replacen("X", c, 1);
                    andstr = andstr.replacen("X", c, 1);
                }
                let or = i64::from_str_radix(&orstr, 2).unwrap();
                let and = i64::from_str_radix(&andstr, 2).unwrap();
                ors.push((or, and));
            }
        } else {
            let pos: i64 = caps.get(3).unwrap().as_str().parse().unwrap();
            let val: i64 = caps.get(4).unwrap().as_str().parse().unwrap();
            for (or, and) in &ors {
                let position = (pos | or) & and;
                mem.insert(position, val);
            }
        }
    }
    let total: i64 = mem.values().sum();
    println!("{}", total);
}
