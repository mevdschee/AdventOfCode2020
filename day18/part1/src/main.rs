use regex::Regex;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn calc(s: String) -> String
{
    let mut operator = "+";
    let mut result = 0;
    for (i,token) in s.split(" ").enumerate() {
        if i%2==0 {
            let num:i64 = token.parse().unwrap();
            result = match operator {
                "+" => result + num,
                "*" => result * num,
                _ => unreachable!("invalid operator"),
            }
        } else { //operator
            operator = token;
        }
    }
    return result.to_string();
}

fn step(s:String) -> String
{
    let re = Regex::new(r"(.*)\(([^\)]+)\)(.*)").unwrap();
    let captures = re.captures(&s);
    if captures.is_none() {
        return s;
    }
    let c = captures.unwrap();
    let search = c[0].to_owned();
    let replace = format!("{}{}{}",c[1].to_owned(),calc(c[2].to_owned()),c[3].to_owned());
    return s.replacen(&search,&replace,1);
}

fn simplify(s:String) -> String
{
    let mut expression = format!("({})", s);
    let mut new_expression:String;
    loop {
        new_expression = step(expression.to_string());
        if new_expression.eq(&expression) {
            break;
        }
        expression = new_expression;
    }
    return expression;
}

fn main() {
    let file = File::open("input").expect("cannot open file");
    let file = BufReader::new(file);
    let mut i:i64 = 0;
    for line in file.lines() {
        let n:i64 = simplify(line.unwrap()).parse().unwrap();
        i += n;
    }
    println!("{}",i);
}
