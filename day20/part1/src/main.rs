use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    // state
    let mut tiles = HashMap::new();
    // parse
    let mut file = File::open("input").expect("cannot open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("cannot read file");
    for tile in content.trim().split("\n\n") {
        let mut parts = tile.trim().split(":");
        let header = parts.next().unwrap();
        let number: i64 = header[5..].parse().unwrap();
        let lines = parts.next().unwrap().trim().split("\n");
        let mut image_lines = vec![];
        for line in lines {
            image_lines.push(line);
        }
        let size = image_lines.len();
        let mut images = vec![];

        let mut image = vec![];
        let mut image_reverse = vec![];
        for line in &image_lines {
            let mut number = 0;
            let mut number_reverse = 0;
            for (i, c) in line.chars().enumerate() {
                let bit = match c {
                    '#' => 1,
                    _ => 0,
                };
                number |= bit << size - 1 - i;
                number_reverse |= bit << i;
            }
            image.push(number);
            image_reverse.push(number_reverse);
        }
        let image_flipped: Vec<i32> = image.iter().rev().map(|n| n.to_owned()).collect();
        let image_reverse_flipped: Vec<i32> =
            image_reverse.iter().rev().map(|n| n.to_owned()).collect();

        images.push(image);
        images.push(image_reverse);
        images.push(image_flipped);
        images.push(image_reverse_flipped);

        let mut image = vec![];
        let mut image_reverse = vec![];
        for col in 0..size {
            let mut number = 0;
            let mut number_reverse = 0;
            for (i, line) in image_lines.iter().enumerate() {
                let c = line.chars().skip(col).next().unwrap();
                let bit = match c {
                    '#' => 1,
                    _ => 0,
                };
                number |= bit << size - 1 - i;
                number_reverse |= bit << i;
            }
            image.push(number);
            image_reverse.push(number_reverse);
        }
        let image_flipped: Vec<i32> = image.iter().rev().map(|n| n.to_owned()).collect();
        let image_reverse_flipped: Vec<i32> =
            image_reverse.iter().rev().map(|n| n.to_owned()).collect();

        images.push(image);
        images.push(image_reverse);
        images.push(image_flipped);
        images.push(image_reverse_flipped);

        tiles.insert(number, images);
    }
    // evaluate
    let mut scores = HashMap::new();
    let mut minimum = -1;
    for (number0, images0) in &tiles {
        let mut score = 0;
        for image0 in images0 {
            for (number1, images1) in &tiles {
                if number0 == number1 {
                    continue;
                }
                for image1 in images1 {
                    if image1[0] == image0[0] {
                        score += 1;
                    }
                }
            }
        }
        if minimum == -1 || score < minimum {
            minimum = score;
        }
        if !scores.contains_key(&score) {
            scores.insert(score, vec![number0]);
        } else {
            scores.get_mut(&score).unwrap().push(number0);
        }
    }
    let i = scores[&minimum].iter().fold(1, |sum, val| sum * *val);
    println!("{:?}", i);
}
