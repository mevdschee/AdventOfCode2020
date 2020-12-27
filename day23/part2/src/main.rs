use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input").expect("cannot open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("cannot read file");
    let mut digits = HashMap::new();
    let mut prev = 0;
    for digit in content.trim().chars() {
        let n:i32 = digit.to_string().parse().unwrap();
        digits.insert(prev, n);
        prev = n;
    }
    for i in 10..=1_000_000 {
        digits.insert(prev,i);
        prev = i;
    }
    digits.insert(prev,digits[&0]);
    for _ in 0..10_000_000 {
        let c = digits.get(&0).unwrap().to_owned();
        let c1 = digits.get(&c).unwrap().to_owned();
        let c2 = digits.get(&c1).unwrap().to_owned();
        let c3 = digits.get(&c2).unwrap().to_owned();
        let c4 = digits.get(&c3).unwrap().to_owned();
        let sanitize = |i| match i { i if i<=0 => 1_000_000, _ => i };
        let mut i = sanitize(c-1);
        let taken = vec![c1,c2,c3];
        while taken.contains(&i) {
            i = sanitize(i-1);
        }
        let i1 = digits.get(&i).unwrap().to_owned();
        let m0 = digits.get_mut(&0).unwrap();
        *m0 = c4;
        let mc = digits.get_mut(&c).unwrap();
        *mc = c4;
        let mi = digits.get_mut(&i).unwrap();
        *mi = c1;
        let mc3 = digits.get_mut(&c3).unwrap();
        *mc3 = i1;
    }
    let i1 = digits[&1];
    let i2 = digits[&i1];
    print!("{}",(i1 as i64) * (i2 as i64));
}