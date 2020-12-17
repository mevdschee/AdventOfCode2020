use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    // state
    let mut field = HashMap::new();
    // parse
    let file = File::open("input").expect("cannot open file");
    let file = BufReader::new(file);
    let z = 0 as i32;
    for (i, line) in file.lines().enumerate() {
        let y = i as i32;
        for (j, c) in line.unwrap().chars().enumerate() {
            let x = j as i32;
            if c == '#' {
                let key = (x, y, z);
                field.insert(key.to_owned(), true);
            }
        }
    }
    // simulate
    for _ in 0..6 {
        let min = field.keys().fold((0, 0, 0), |m, v| {
            (
                cmp::min(m.0, v.0 - 1),
                cmp::min(m.1, v.1 - 1),
                cmp::min(m.2, v.2 - 1),
            )
        });
        let max = field.keys().fold((0, 0, 0), |m, v| {
            (
                cmp::max(m.0, v.0 + 1),
                cmp::max(m.1, v.1 + 1),
                cmp::max(m.2, v.2 + 1),
            )
        });
        let mut next = HashMap::new();
        for z in min.2..=max.2 {
            for y in min.1..=max.1 {
                for x in min.0..=max.0 {
                    let value = field.contains_key(&(x, y, z));
                    let mut active = 0;
                    for dx in -1..=1 {
                        for dy in -1..=1 {
                            for dz in -1..=1 {
                                if dx == 0 && dy == 0 && dz == 0 {
                                    continue;
                                }
                                if field.contains_key(&(x + dx, y + dy, z + dz)) {
                                    active += 1;
                                }
                            }
                        }
                    }
                    let nextvalue;
                    if value {
                        nextvalue = active == 2 || active == 3;
                    } else {
                        nextvalue = active == 3;
                    }
                    if nextvalue {
                        next.insert((x, y, z).to_owned(), nextvalue);
                    }
                }
            }
        }
        field = next;
    }
    println!("{}", field.values().count());
}
