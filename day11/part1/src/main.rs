use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    let file = File::open("input").expect("cannot open file");
    let file = BufReader::new(file);
    let mut seats = HashMap::<(i32,i32),bool>::new();
    for (y, line) in file.lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            if c=='L' {
                seats.insert((x as i32,y as i32), false);
            }
        }
    }
    let mut changed = true;
    while changed {
        let mut next = HashMap::<(i32,i32),bool>::new();
        changed = false;
        for (key,value) in &seats {
            let (x,y) = key;
            let mut occupied = 0;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx==0 && dy ==0 {
                        continue;
                    }
                    if seats.contains_key(&(x+dx,y+dy)) {
                        if seats[&(x+dx,y+dy)] {
                            occupied += 1;
                        } 
                    }
                }
            }
            let mut nextvalue = *value;
            if *value {
                if occupied>=4 {
                    nextvalue = false;
                    changed = true;
                }
            } else {
                if occupied==0 {
                    nextvalue = true;
                    changed = true;
                }
            }
            next.insert(*key, nextvalue);
        }
        seats = next;
    }
    println!("{}",seats.values().filter(|&n|*n).count());
}
