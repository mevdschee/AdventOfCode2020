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
    let mut diff1 = 0;
    let mut diff3 = 0;
    let mut j = 0;
    for i in 0..numbers.len() {
        let n = numbers[i];
        if n>=j+1 && n<=j+3 {
            if n==j+1 {
                diff1 += 1;
            } else if n==j+3 {
                diff3+=1;
            }
            j = n;
        }
    }
    println!("{}", diff1*(diff3+1));
}
