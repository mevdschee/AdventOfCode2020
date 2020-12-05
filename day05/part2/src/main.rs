use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::cmp;
use std::collections::HashMap;

fn main() {
    let file = File::open("input").expect("cannot open file");
    let file = BufReader::new(file);
    let mut i = 0;
    let mut max = 0;
    let mut numbers = HashMap::new();
    for line in file.lines() {
        let s = line.unwrap().replace("B", "1").replace("F", "0").replace("R", "1").replace("L", "0");
        let intval = isize::from_str_radix(&s, 2).unwrap();
        numbers.insert(intval,true);
        max = cmp::max(max, intval);
    }
    for num in 8..=(max-8) {
        if !numbers.contains_key(&num) {
            i = num;
            break;
        }
    }   
    println!("{}", i); 
}
