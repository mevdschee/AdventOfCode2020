use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn make_regex(rules:&HashMap<i32,Vec<Vec<i32>>>, rule:i32, depth: i32) -> String
{
    if depth>20 {
        return "".to_owned();
    }
    let subrules = &rules[&rule];
    let mut expressions = vec![];
    for subrule in subrules {
        let mut numbers = vec![];
        for number in subrule {
            if *number == -1 {
                numbers.push("a".to_owned());
            } else if *number == -2 {
                numbers.push("b".to_owned());
            } else {
                numbers.push(make_regex(rules, *number, depth+1));
            }
        }
        expressions.push(numbers.join(""));
    }
    if expressions.len()>1 {
        return format!("({})",expressions.join("|"));
    } else {
        return format!("{}",expressions[0]);
    }
}

fn main() {
    // state
    let mut rules = HashMap::new();
    let mut messages = HashSet::new();
    // parse
    let file = File::open("input").expect("cannot open file");
    let file = BufReader::new(file);
    let lines = file.lines();
    let mut part = 1;
    for line in lines {
        let s = line.unwrap();
        if s == "" {
            part+=1;
            continue;
        }
        if part==1 {
            let mut parts = s.split(": ");
            let i:i32 = parts.next().unwrap().parse().unwrap();
            let subrules = parts.next().unwrap().split(" | ");
            let mut options = vec![];
            for subrule in subrules {
                let mut option = vec![];
                for number in subrule.split(" ") {
                    let n:i32;
                    if number == "\"a\"" {
                        n = -1;
                    } else if number == "\"b\"" {
                        n = -2;
                    } else { 
                        n = number.parse().unwrap();
                    }
                    option.push(n);
                }
                options.push(option);
            }
            rules.insert(i,options);
        } else {
            messages.insert(s);
        }
    }
    // change rules
    rules.insert(8,vec![vec![42],vec![42,8]]);
    rules.insert(11,vec![vec![42,31],vec![42,11,31]]);
    // evaluate
    let expression = make_regex(&rules,0,0);
    let re = Regex::new(&format!("^{}$",&expression)).unwrap();
    let mut i = 0;
    for message in messages {
        if re.is_match(&message) {
            i += 1;
        }
    }
    println!("{}",i);
}
