use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::VecDeque;

fn combat(hands: &Vec<VecDeque<i32>>) -> (i32, i32)
{
    let mut hands = hands.clone();
    let mut win = 0;
    let mut games = HashSet::new();
    while hands[0].len()>0 && hands[1].len()>0 {
        let game = format!("{:?}",hands);
        let card0 = hands[0].pop_front().unwrap();
        let card1 = hands[1].pop_front().unwrap();
        //println!("{} vs {} for {:?}",card0, card1, hands);
        if games.contains(&game) {
            win = 0;
        } else {
            games.insert(game);
            if (card0 as usize) <= hands[0].len() && (card1 as usize) <= hands[1].len() {
                let hand0: VecDeque<i32> = hands[0].iter().take(card0 as usize).cloned().collect();
                let hand1: VecDeque<i32> = hands[1].iter().take(card1 as usize).cloned().collect();
                let hands = vec![hand0,hand1];
                win = combat(&hands).0;
            } else if card0>card1 {
                win = 0;
            } else {
                win = 1;
            }
        }
        if win==0 {
            hands[0].push_back(card0);
            hands[0].push_back(card1);
        } else {
            hands[1].push_back(card1);
            hands[1].push_back(card0);
        }
    }
    let mut sum = 0;
    for i in 1..=hands[win as usize].len() {
        let number = hands[win as usize].pop_back().unwrap();
        sum += i as i32 * number;
    }
    return (win,sum);
}

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
    let sum = combat(&hands).1;
    println!("{}", sum);
}

// 32873, 30999