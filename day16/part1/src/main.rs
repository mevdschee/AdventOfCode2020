use std::collections::HashMap;
use std::io::Read;
use std::fs::File;

fn main() {
    // state
    let mut fields = HashMap::new();
    let mut your_ticket = vec![];
    let mut nearby_tickets = vec![];
    // parse
    let mut file = File::open("input").expect("cannot open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("cannot read file");
    let mut parts = content.trim().split("\n\n");
    let part = parts.next().unwrap();
    for line in part.split("\n") {
        let mut parts = line.split(": ");
        let name = parts.next().unwrap();
        let mut rules = vec![];
        for rule in parts.next().unwrap().split(" or ") {
            let mut parts = rule.split("-");
            let min: i64 = parts.next().unwrap().parse().unwrap();
            let max: i64 = parts.next().unwrap().parse().unwrap();
            rules.push((min,max));
        }
        fields.insert(name,rules);
    }
    let part = parts.next().unwrap();
    for line in part.split("\n").skip(1) {
        for number in line.split(",") {
            let n: i64 = number.parse().unwrap();
            your_ticket.push(n);
        }
    }
    let part = parts.next().unwrap();
    for line in part.split("\n").skip(1) {
        let mut ticket = vec![];
        for number in line.split(",") {
            let n: i64 = number.parse().unwrap();
            ticket.push(n);
        }
        nearby_tickets.push(ticket);
    }
    // evaluate
    let mut i = 0;
    for ticket in nearby_tickets {
        for number in ticket {
            let mut matches = false;
            for rules in fields.values() {
                for rule in rules {
                    let (min,max) = rule;
                    if number>=*min && number<=*max {
                        matches = true;
                    }
                }
            }
            if !matches {
                i += number;
            }
        }
    }
    println!("{}", i);
}