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
            let color: String = cap[2].parse().unwrap();
            if !rules.contains_key(&source.to_string()) {
                rules.insert(source.to_string(), vec![color]);
            } else {
                rules
                    .entry(source.to_string())
                    .and_modify(|e| e.push(color));
            }
        }
    }
    let mut v: Vec<String> = vec!["shiny gold".to_string()];
    let mut count = 0;
    while count < v.len() {
        count = v.len();
        for c in v.clone() {
            for rule in rules.clone() {
                let key = rule.0;
                let entries = rule.1;
                if entries.contains(&c) {
                    if !v.contains(&key.to_string()) {
                        v.push(key.to_string());
                    }
                }
            }
        }
    }
    println!("{}", count - 1);
}
