use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let file = File::open("input").expect("cannot open file");
    let file = BufReader::new(file);
    let mut s = 0;
    for line in file.lines() {
        let content = line.unwrap();
        let inwoners:i64 = content.trim().replace(".","").parse().unwrap();
        let mut i = 1;
        loop {
            let corner:i64 = (1..=i).sum();
            let square = (i*3)*(i*3);
            let n = square - corner*4;
            if n>=inwoners {
                break;
            }
            i+=1;
        }    
        s += 8 * i;
    }
    println!("{}",s);
}
