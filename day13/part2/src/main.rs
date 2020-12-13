use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input").expect("cannot open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("cannot read file");
    let lines = content.splitn(2, "\n");
    let ids = lines.skip(1).next().unwrap().trim().split(',');
    let mut step = 1;
    let mut t = 0;
    for (pos,id) in ids.enumerate() {
        if id=="x" {
            continue;
        }
        let bus:i64 = id.parse().unwrap();
        while (t + pos as i64) % bus != 0 {
            t += step;
        }
        step *= bus;
    }
    println!("{}", t);
}
