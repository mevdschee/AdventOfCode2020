use std::io::Read;
use std::fs::File;

fn main() {
    let mut file = File::open("input").expect("cannot open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("cannot read file");
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
    println!("{}",i);
}
