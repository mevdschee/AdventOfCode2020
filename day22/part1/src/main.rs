use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::VecDeque;

fn main() {
    // state
    let mut hands:Vec<VecDeque<i32>> = vec![VecDeque::new(),VecDeque::new()];
    // parse
    let file = File::open("input").expect("cannot open file");
    let file = BufReader::new(file);
    let mut player = 0;
    for line in file.lines() {
        let s = line.unwrap().to_owned();
        if s=="" {
            player+=1;
            continue;
        }
        let parsed = s.parse();
        if parsed.is_err() {
            continue;
        }
        let number:i32 = parsed.unwrap();
        hands[player].push_back(number);
    }
    // evaluate
    let mut win = 0;
    while hands[0].len()>0 && hands[1].len()>0 {
        let card0 = hands[0].pop_front().unwrap();
        let card1 = hands[1].pop_front().unwrap();
        if card0>card1 {
            win = 0;
            hands[0].push_back(card0);
            hands[0].push_back(card1);
        } else {
            win = 1;
            hands[1].push_back(card1);
            hands[1].push_back(card0);
        }
    }
    let mut sum = 0;
    for i in 1..=hands[win].len() {
        let number = hands[win].pop_back().unwrap();
        sum += i as i32 * number;
    }
    println!("{}", sum);
}
