use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::io::Read;
use std::fs::File;

fn main() {
    // state
    let mut fields = HashMap::new();
    let mut your_ticket = vec![];
    let mut nearby_tickets = vec![];
    let mut collisions = HashMap::new();
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
    // calculate invalid
    let mut remove = vec![];
    for (i, ticket) in nearby_tickets.iter().enumerate() {
        for number in ticket {
            let mut matches = false;
            for rules in fields.values() {
                for rule in rules {
                    let (min,max) = rule;
                    if number>=min && number<=max {
                        matches = true;
                    }
                }
            }
            if !matches {
                remove.push(i);
            }
        }
    }
    // remove invalid
    for i in remove.iter().rev() {
        nearby_tickets.remove(*i);
    }
    // calculate collisions
    for ticket in nearby_tickets {
        for (column, number) in ticket.iter().enumerate() {
            for (name, rules) in fields.iter() {
                let mut matches = false;
                for rule in rules {
                    let (min,max) = rule;
                    if number>=min && number<=max {
                        matches = true;
                    }
                }
                if !matches {
                    if !collisions.contains_key(&column) {
                        collisions.insert(column,HashSet::new());
                    }
                    collisions.get_mut(&column).unwrap().insert(*name);
                }
            }
        }
    }
    // match fields
    let mut mapping_options = HashMap::new();
    for column in 0..fields.len() {
        let keys = HashSet::from_iter(fields.keys().cloned());
        if collisions.contains_key(&column) {
            let diff = keys.difference(&collisions[&column]).cloned().collect();
            mapping_options.insert(column,diff);
        } else {
            mapping_options.insert(column,keys);
        }
    }
    // resolve options
    let mut mapping = HashMap::new();
    for _ in 0..fields.len() {
        for column in 0..fields.len() {
            if mapping_options[&column].len()==1 {
                let first = mapping_options[&column].iter().next().unwrap().to_owned();
                mapping.insert(first,column);
                for options in mapping_options.values_mut() {
                    options.remove(&first);
                }
            }
        }
    }
    // calc answer
    let mut i = 1;
    for (name, column) in mapping {
        if name.len() > 9 && name[0..9].to_string() == "departure" {
            i *= your_ticket[column];
        }
    }
    println!("{:?}", i);
}