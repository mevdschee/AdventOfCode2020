use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input").expect("cannot open file");
    let file = BufReader::new(file);
    let mut numbers: Vec<i64> = Vec::new();
    numbers.push(0);
    for line in file.lines() {
        let s = line.unwrap();
        let number = s.parse().unwrap();
        numbers.push(number);
    }
    numbers.sort();
    numbers.push(numbers[numbers.len()-1]+3);
    // do clever stuff
    let mut options:Vec<i64> = vec![0;numbers.len()];
    options[0] = 1;
    for i in 0..numbers.len() {
        let mut opts = 0;
        for o in 1..=3 {
            if i >= o && numbers[i]-numbers[i-o] <= 3 { 
                opts += options[i-o];
            }
        }
        options[i] += opts;
    }
    println!("{}", options[options.len()-1]);
}
