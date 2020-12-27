use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

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
    println!("{}",tiles.len());
}
