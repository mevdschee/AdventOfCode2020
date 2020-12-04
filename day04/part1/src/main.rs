use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input").expect("cannot open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("cannot read file");
    let mut i = 0;
    for passport in content.split("\n\n") {
        let mut properties = HashMap::new();
        for property in passport.split_whitespace() {
            let mut kv = property.splitn(2, ":");
            let key = kv.next().unwrap();
            let value = kv.next().unwrap();
            if key == "cid" {
                continue;
            }
            if properties.contains_key(key) {
                properties.get(key).replace(&value);
            } else {
                properties.insert(key, value);
            }
        }
        if properties.len() == 7 {
            i += 1;
        }
    }
    println!("{}", i);
}
