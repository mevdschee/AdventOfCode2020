use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input").expect("cannot open file");
    let file = BufReader::new(file);
    let mut numbers: Vec<i64> = Vec::new();
    for line in file.lines() {
        let s = line.unwrap();
        let number = s.parse().unwrap();
        numbers.push(number);
    }
    numbers.sort();
    let mut used: Vec<i64> = Vec::new();
    let mut j = 0;
    for i in 0..numbers.len() {
        let n = numbers[i];
        if n>=j+1 && n<=j+3 {
            used.push(n);
            j = n;
        }
    }
    let l = used.len();
    let options = 1;
    for i in 0..l {
        let n = numbers[l-i];
    }
    println!("{}", options);
}
