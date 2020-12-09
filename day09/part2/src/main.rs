use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input").expect("cannot open file");
    let file = BufReader::new(file);
    let preamble = 25;
    let mut numbers: Vec<i64> = Vec::new();
    let mut i = 0;
    let mut number = 0 ;
    for line in file.lines() {
        let s = line.unwrap();
        number = s.parse().unwrap();
        if i>=preamble {
            let mut found = false;
            for j in 0..preamble {
                for k in j..preamble {
                    let numj = numbers[numbers.len()-1-j];
                    let numk = numbers[numbers.len()-1-k];
                    if numj+numk == number {
                        found = true;
                    }
                    if found {
                        break;
                    }
                }
                if found {
                    break;
                }
            }
            if !found {
                break;
            }  
        }
        numbers.push(number);
        i += 1;
    }
    let mut begin = 0;
    let mut end= 0;
    let mut sum = 0;
    while sum != number {
        if sum > number {
            begin += 1;
        } else if sum<number {
            end += 1;
        }
        sum = numbers[begin..end].iter().sum();
    }
    let max = numbers[begin..end].iter().max().unwrap();
    let min = numbers[begin..end].iter().min().unwrap();
    println!("{}", min + max);
}
