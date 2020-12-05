use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::cmp;

fn main() {
    let file = File::open("input").expect("cannot open file");
    let file = BufReader::new(file);
    let mut i = 0;
    for line in file.lines() {
        let s = line.unwrap().replace("B", "1").replace("F", "0").replace("R", "1").replace("L", "0");
        let intval = isize::from_str_radix(&s, 2).unwrap();
        i = cmp::max(i, intval);
    }
    println!("{}", i);
}
