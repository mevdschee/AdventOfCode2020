use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut vec = Vec::new();

    let file = File::open("input").expect("cannot open file");
    let file = BufReader::new(file);
    for line in file.lines() {
        let s: String = line.ok().unwrap();
        let num: i32 = s.parse().unwrap();
        vec.push(num);
    }
    for num1 in &vec {
        for num2 in &vec {
            for num3 in &vec {
                if num1 + num2 + num3 == 2020 {
                    println!("{}+{}+{}={}", num1, num2, num3, num1 + num2 + num3);
                    println!("{}*{}*{}={}", num1, num2, num3, num1 * num2 * num3);
                }
            }
        }
    }
}
