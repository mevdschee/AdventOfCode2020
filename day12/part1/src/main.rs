use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input").expect("cannot open file");
    let file = BufReader::new(file);
    let mut position = (0,0);
    let mut direction = (1,0);
    for line in file.lines() {
        let s = line.unwrap();
        let (letter,number) = s.split_at(1);
        let n:i32 = number.parse().unwrap();
        direction = match (letter,n) {
            ("L",90) | ("R",270) => (direction.1,-direction.0),
            ("R",90) | ("L",270) => (-direction.1,direction.0),
            ("L",180) | ("R",180) => (-direction.0,-direction.1),
            _ => direction
        };
        position = match letter {
            "N" => (position.0,position.1-n),
            "S" => (position.0,position.1+n),
            "E" => (position.0+n,position.1),
            "W" => (position.0-n,position.1),
            "F" => (position.0+direction.0*n,position.1+direction.1*n),
            _ => position,
        };
    }
    println!("{}",position.0.abs()+position.1.abs());
}
