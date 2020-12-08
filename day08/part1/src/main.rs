use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input").expect("cannot open file");
    let file = BufReader::new(file);
    let mut code = Vec::new();
    for line in file.lines() {
        let s = line.unwrap();
        let mut parts = s.split(" ");
        let opcode: String = parts.next().unwrap().to_string();
        let argument = parts.next().unwrap();
        let argnumber: i32 = argument.parse().unwrap();
        code.push((opcode.clone(), argnumber));
    }
    let mut visited = HashMap::new();
    let mut pc = 0;
    let mut acc = 0;
    while !visited.contains_key(&pc) {
        let entry = code.get(pc as usize).unwrap();
        let (opcode, argnumber) = entry;
        visited.insert(pc, true);
        pc = match &**opcode {
            "jmp" => pc + argnumber,
            "acc" => {
                acc += argnumber;
                pc + 1
            }
            _ => pc + 1,
        };
    }
    println!("{}", acc);
}
