use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input").expect("cannot open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("cannot read file");
    let mut numbers = vec![];
    let mut last = 0;
    for number in content.trim().split(",") {
        last = number.parse().unwrap();
        numbers.push(last);
    }
    for i in numbers.len()..30000000 {
        let pos = numbers.iter().rev().skip(1).position(|&n| n == last);
        if pos.is_some() {
            last = i - (numbers.len() - 1 - pos.unwrap());
        } else {
            last = 0;
        }
        numbers.push(last);
    }
    println!("{}", last);
}
