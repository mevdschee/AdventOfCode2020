use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn print_tile(tiles: &HashMap<i32, Vec<Vec<i32>>>, tile: i32, rotation: usize) {
    for n in &tiles[&tile][rotation] {
        println!("{:#012b}", n);
    }
}

fn tile_connects(tiles: &HashMap<i32, Vec<Vec<i32>>>, tile0: i32, tile1: i32) -> i32 {
    let mut found = false;
    let mut i = 0;
    for side0 in &tiles[&tile0] {
        for side1 in &tiles[&tile1] {
            if side0[0 as usize] == side1[0 as usize] {
                found = true;
                break;
            }
            i += 1;
        }
        if found {
            break;
        }
    }
    return i;
}

fn find_next_tile(
    tiles: &HashMap<i32, Vec<Vec<i32>>>,
    image: &Vec<i32>,
    scores: &HashMap<i32, Vec<i32>>,
) -> i32 {
    let y = image.len() / 12;
    let x = image.len() % 12;
    let mut pool = 12;
    if y > 0 && y < 11 {
        pool += 2;
    }
    if x > 0 && x < 11 {
        pool += 2;
    }
    for tile in &scores[&pool] {
        if image.contains(tile) {
            continue;
        }
        if y == 0 || tile_connects(tiles, *tile, image[(y - 1) * 12 + x]) > 0 {
            if x == 0 || tile_connects(tiles, *tile, image[y * 12 + x - 1]) > 0 {
                return *tile;
            }
        }
    }
    return 0;
}

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
        let number: i32 = header[5..].parse().unwrap();
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

        let images = vec![
            images[0].clone(), // 0 normal
            images[6].clone(), // 90 clock-wise
            images[3].clone(), // 180 upside down
            images[5].clone(), // 270 clock-wise
            images[1].clone(), // 0 mirror (horizontally)
            images[4].clone(), // 90 clock-wise mirror
            images[2].clone(), // 180 upside down mirror
            images[7].clone(), // 270 clock-wise mirror
        ];

        tiles.insert(number, images);
    }
    // evaluate
    let mut scores = HashMap::new();
    let mut minimum = -1;
    for (number, images) in &tiles {
        let mut score = 0;
        for image in images {
            let border = image[0];
            for (_number, images) in &tiles {
                for image in images {
                    if image[0] == border {
                        score += 1;
                    }
                }
            }
        }
        if minimum == -1 || score < minimum {
            minimum = score;
        }
        if !scores.contains_key(&score) {
            scores.insert(score, vec![*number]);
        } else {
            scores.get_mut(&score).unwrap().push(*number);
        }
    }
    // build image
    let mut image = vec![];
    for _ in 0..144 {
        image.push(find_next_tile(&tiles, &image, &scores));
    }
    for i in 0..2 {
        println!("{}", image[i]);
        print_tile(&tiles, image[i], 0);
    }
    //println!("{:?}", image);
}
