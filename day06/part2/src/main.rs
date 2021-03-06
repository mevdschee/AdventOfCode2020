use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input").expect("cannot open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("cannot read file");
    let mut i = 0;
    for group in content.trim().split("\n\n") {
        let mut questions = HashMap::new();
        let mut p = 0;
        for person in group.split("\n") {
            for question in person.chars() {
                let v = questions.entry(question).or_insert(0);
                *v += 1;
            }
            p += 1;
        }
        for (&_k,&v) in questions.iter() {
            if v == p {
                i+=1;
            }
        }
    }
    println!("{}", i);
}
