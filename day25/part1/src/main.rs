use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let file = File::open("input").expect("cannot open file");
    let file = BufReader::new(file);
    let mut lines = file.lines();
    let public_key_1: i64 = lines.next().unwrap().unwrap().parse().unwrap();
    let public_key_2: i64 = lines.next().unwrap().unwrap().parse().unwrap();
    let subject_number = 7;
    let mut value = 1;
    let mut loop_size_1 = 0;
    for i in 1..20201227 {
        value *= subject_number;
        value %= 20201227;
        if value == public_key_1 {
            loop_size_1 = i;
        }
    }
    value = 1;
    for _ in 0..loop_size_1 {
        value *= public_key_2;
        value %= 20201227;
    }
    println!("{}",value);
}
