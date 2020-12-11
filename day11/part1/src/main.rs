use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    let file = File::open("input").expect("cannot open file");
    let file = BufReader::new(file);
    let mut seats = HashMap::<(i32,i32),bool>::new();
    let mut maxy:i32 = 0;
    let mut maxx:i32 = 0;
    for (y, line) in file.lines().enumerate() {
        maxy = max(maxy,y as i32);
        for (x, c) in line.unwrap().chars().enumerate() {
            maxx = max(maxx,x as i32);
            if c=='L' {
                seats.insert((x as i32,y as i32), false);
            }
        }
    }
    let size = max(maxx,maxy)+1;
    let mut changed = true;
    while changed {
        let mut next = HashMap::<(i32,i32),bool>::new();
        changed = false;
        for x in 0..size {
            for y in 0..size {
                if seats.contains_key(&(x,y)) {
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
                    let mut value = seats[&(x,y)];
                    if seats[&(x,y)] {
                        if occupied>=4 {
                            value = false;
                            changed = true;
                        }
                    } else {
                        if occupied==0 {
                            value = true;
                            changed = true;
                        }
                    }
                    next.insert((x,y), value);
                    
                }
            }
        }
        seats = next;
    }
    let mut i = 0;
    for k in seats.keys() {
        if seats[k] {
            i+=1;
        }
    }
    println!("{}",i);
}
