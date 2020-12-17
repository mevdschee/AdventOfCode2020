use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input").expect("cannot open file");
    let file = BufReader::new(file);
    let mut seats = HashMap::new();
    let mut maxy: i32 = 0;
    let mut maxx: i32 = 0;
    for (y, line) in file.lines().enumerate() {
        maxy = max(maxy, y as i32);
        for (x, c) in line.unwrap().chars().enumerate() {
            maxx = max(maxx, x as i32);
            if c == 'L' {
                seats.insert((x as i32, y as i32), false);
            }
        }
    }
    let size = max(maxx, maxy) + 1;
    let mut changed = true;
    while changed {
        let mut next = HashMap::new();
        changed = false;
        for (key, value) in &seats {
            let (x, y) = key;
            let mut occupied = 0;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    for m in 1..size {
                        let nx = x + dx * m;
                        let ny = y + dy * m;
                        if nx < 0 || ny < 0 || nx >= size || ny >= size {
                            break;
                        }
                        if seats.contains_key(&(nx, ny)) {
                            if seats[&(nx, ny)] {
                                occupied += 1;
                            }
                            break;
                        }
                    }
                }
            }
            let mut nextvalue = *value;
            if *value {
                if occupied >= 5 {
                    nextvalue = false;
                    changed = true;
                }
            } else {
                if occupied == 0 {
                    nextvalue = true;
                    changed = true;
                }
            }
            next.insert(*key, nextvalue);
        }
        seats = next;
    }
    println!("{}", seats.values().filter(|&n| *n).count());
}
