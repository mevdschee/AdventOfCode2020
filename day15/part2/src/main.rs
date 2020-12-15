use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input").expect("cannot open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("cannot read file");
    let mut numbers = HashMap::new();
    let mut last = 0;
    for (i,number) in content.trim().split(",").enumerate() {
        if i>0 { 
            numbers.insert(last,i);
        }
        last = number.parse().unwrap();
    }
    for i in (numbers.len()+1)..30000000 {
        let pos = numbers.get(&last);
        let nlast = last;
        if pos.is_some() {
            last = i - pos.unwrap();
        } else {
            last = 0;
        }
        numbers.insert(nlast,i);
    }
    println!("{}", last);
}
