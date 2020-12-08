use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input").expect("cannot open file");
    let file = BufReader::new(file);
    let mut rules = HashMap::new();
    for line in file.lines() {
        let s = line.unwrap();
        let mut parts = s.split(" bags contain ");
        let source = parts.next().unwrap();
        let targets = parts.next().unwrap();
        let re = Regex::new(r"([0-9]) ([a-z ]+) bags?").unwrap();
        for cap in re.captures_iter(&targets) {
            let count: i32 = cap[1].parse().unwrap();
            let color: String = cap[2].parse().unwrap();
            if !rules.contains_key(&source.to_string()) {
                rules.insert(source.to_string(), vec![(count, color)]);
            } else {
                rules
                    .entry(source.to_string())
                    .and_modify(|e| e.push((count, color)));
            }
        }
    }
    let mut v: Vec<(i32, String)>;
    let mut v2: Vec<(i32, String)> = vec![(1, "shiny gold".to_string())];
    let mut i = 0;
    while v2.len() > 0 {
        v = v2;
        v2 = vec![];
        for (count, color) in v {
            for rule in rules.clone() {
                if rule.0 == color {
                    for (count2, color2) in rule.1 {
                        v2.push((count * count2, color2));
                        i += count * count2;
                    }
                }
            }
        }
    }
    println!("{}", i);
}
