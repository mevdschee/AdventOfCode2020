use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input").expect("cannot open file");
    let file = BufReader::new(file);
    let mut position = (0,0);
    let mut waypoint = (10,-1);
    for line in file.lines() {
        let s = line.unwrap();
        let (letter,number) = s.split_at(1);
        let n:i64 = number.parse().unwrap();
        waypoint = match (letter,n) {
            ("L",90) | ("R",270) => (waypoint.1,-waypoint.0),
            ("R",90) | ("L",270) => (-waypoint.1,waypoint.0),
            ("L",180) | ("R",180) => (-waypoint.0,-waypoint.1),
            ("L",_) | ("R",_) => panic!("Unexpected angle: {}",n),
            ("N",_) => (waypoint.0,waypoint.1-n),
            ("S",_) => (waypoint.0,waypoint.1+n),
            ("E",_) => (waypoint.0+n,waypoint.1),
            ("W",_) => (waypoint.0-n,waypoint.1),
            _ => waypoint
        };
        position = match letter {
            "F" => (position.0+waypoint.0*n,position.1+waypoint.1*n),
            _ => position,
        };
    }
    println!("{}",position.0.abs()+position.1.abs());
}
