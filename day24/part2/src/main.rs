use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::cmp;

/**
 * pointy top, axial system
 * nw =  0,-1
 * ne = +1,-1
 * w  = -1, 0
 * e  = +1, 0
 * sw = -1,+1
 * se =  0,+1
 */

fn main() {
    let file = File::open("input").expect("cannot open file");
    let file = BufReader::new(file);
    let mut tiles = HashMap::new();
    for line in file.lines() {
        let line = line.unwrap();
        let mut chars = line.chars();
        let mut q = 0;
        let mut r = 0;
        loop {
            let c1 = chars.next();
            if c1.is_none() {
                break;
            }
            let (dq,dr) = match c1.unwrap() {
                'n' | 's' => {
                    let c2 = chars.next();
                    match (c1.unwrap(), c2.unwrap()) {
                        ('n','w') => (0,-1),
                        ('n','e') => (1,-1),
                        ('s','w') => (-1,1),
                        ('s','e') => (0,1),
                        _ => (0,0)
                    }
                }
                'w' => (-1,0),
                'e' => (1,0),
                _ => (0,0),
            };
            q += dq;
            r += dr;
        }
        let key = (q,r);
        if tiles.contains_key(&key) {
            tiles.remove(&key);
        } else {
            tiles.insert(key,true);
        }
    }
    let neighbours = vec![(0,-1),(1,-1),(-1,1),(0,1),(-1,0),(1,0)];
    for _ in 0..100 {
        let mut next = HashMap::new();
        let min = tiles.keys().fold((0, 0), |m, v| {
            (
                cmp::min(m.0, v.0 - 1),
                cmp::min(m.1, v.1 - 1),
            )
        });
        let max = tiles.keys().fold((0, 0), |m, v| {
            (
                cmp::max(m.0, v.0 + 1),
                cmp::max(m.1, v.1 + 1),
            )
        });
        for q in (min.0-1)..=(max.0+1) {
            for r in (min.1-1)..=(max.1+1) {
                let mut count = 0;
                for (dq,dr) in &neighbours {
                    let key = (q+dq,r+dr);
                    if tiles.contains_key(&key) {
                        count+=1;
                    }
                }
                let key = (q,r);
                let mut value = tiles.contains_key(&key);
                if value {
                    if count == 0 || count>2 {
                        value = false;
                    } 
                } else {
                    if count == 2 {
                        value = true;
                    }
                }
                if value {
                    next.insert(key, true);
                }
            }  
        }
        tiles = next;
    }
    println!("{}",tiles.len());
}
