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
                for k in 0..preamble {
                    let numj = numbers[numbers.len()-1-j];
                    let numk = numbers[numbers.len()-1-k];
                    if j!=k && numj+numk == number {
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
    let mut found = false;
    for o in 0..numbers.len() {
        for l in 0..(numbers.len()-o) {
            let sum: i64 = numbers[o..o+l].iter().sum();
            if sum > number {
                break;
            }
            if sum == number {
                let max = numbers[o..o+l].iter().max().unwrap();
                let min = numbers[o..o+l].iter().min().unwrap();
                number = min + max;
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
    println!("{}",number);
}
