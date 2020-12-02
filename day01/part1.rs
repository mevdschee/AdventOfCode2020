use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let mut vec = Vec::new();
        
    let file = File::open("input").expect("cannot open file");
    let file = BufReader::new(file);
    for line in file.lines() {
        let num: i32 = line.ok().unwrap().parse().unwrap();
        for v in &vec {
            if v+num==2020 {
                println!("{}+{}=2020",v,num);
                println!("{}*{}={}",v,num,v*num);
            }
        }
        vec.push(num);
    }

}
