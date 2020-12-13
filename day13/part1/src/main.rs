use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input").expect("cannot open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("cannot read file");
    let mut lines = content.splitn(2, "\n");
    let start:i64 = lines.next().unwrap().parse().unwrap();
    let ids = lines.next().unwrap().trim().split(',');
    let mut best_bus = 0;
    let mut best_wait = 0;
    for id in ids {
        if id=="x" {
            continue;
        }
        let bus:i64 = id.parse().unwrap();
        let wait = (bus - start % bus) % bus;
        if best_bus == 0 || wait<best_wait {
            best_bus = bus;
            best_wait = wait
        }
    }
    println!("{}", best_bus*best_wait);
}
