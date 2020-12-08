use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

fn main() {
    let file = File::open("input").expect("cannot open file");
    let file = BufReader::new(file);
    let mut code = Vec::new();
    for line in file.lines() {
        let s = line.unwrap();
        let mut parts = s.split(" ");
        let opcode = parts.next().unwrap().to_string();
        let argument = parts.next().unwrap();
        let argnumber = argument.parse().unwrap();
        code.push((opcode.clone(), argnumber));
    }
    for corrupt in 0..code.len() {
        let mut code2 = code.clone();
        if let Some(entry) = code2.get_mut(corrupt) {
            let (opcode, argnumber) = entry;
            if opcode == "jmp" {
                *entry = ("nop".to_string(), *argnumber);
            } else if opcode == "nop" {
                *entry = ("jmp".to_string(), *argnumber);
            }
        }
        let mut visited = HashMap::new();
        let mut pc = 0;
        let mut acc = 0;
        while !visited.contains_key(&pc) {
            let entry = code2.get(pc as usize).unwrap();
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
            if pc as usize == code2.len() {
                println!("{}", acc);
                exit(0);
            }
        }
    }
}
