use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

// fn print_tile(tiles: &HashMap<i32, Vec<Vec<i32>>>, tile: i32, rotation: usize) {
//     println!("{} {}", tile, rotation);
//     for n in &tiles[&tile][rotation] {
//         let s = format!("{:#012b}", n);
//         println!(
//             "{}",
//             s[2..].to_string().replace("0", "..").replace("1", "()")
//         );
//     }
// }


// fn print_bitmap(bitmap: &Vec<i128>) {
//     for n in bitmap {
//         let s = format!("{:#098b}", n);
//         println!(
//             "{}",
//             s[2..].to_string().replace("0", ".").replace("1", "#")
//         );
//     }
// }

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
            images[0].clone(), // 0 => 0 normal
            images[6].clone(), // 1 => 90 counter-clock-wise
            images[3].clone(), // 2 => 180 upside down
            images[5].clone(), // 3 => 270 counter-clock-wise
            images[1].clone(), // 4 => 0 mirror (around vertical axis)
            images[4].clone(), // 5 => 90 counter-clock-wise mirror
            images[2].clone(), // 6 => 180 upside down mirror
            images[7].clone(), // 7 => 270 counter-clock-wise mirror
        ];

        tiles.insert(number, images);
        // for i in 0..8{
        //     print_tile(&tiles,number,i);
        //     print_tile(&tiles,number,(i+6)%8);
        //     println!("");
        // }
        // panic!("exit");
    }
    // evaluate
    let mut scores = HashMap::new();
    let mut connects = HashMap::new();
    for (number0, images0) in &tiles {
        let mut score = 0;
        for (orientation0, image0) in images0.iter().enumerate() {
            for (number1, images1) in &tiles {
                if number0 == number1 {
                    continue;
                }
                for (orientation1, image1) in images1.iter().enumerate() {
                    if image0[0] == image1[0] {
                        score += 1;
                        connects.insert(
                            (number0.to_owned(), orientation0),
                            (number1.to_owned(), orientation1),
                        );
                    }
                }
            }
        }
        if !scores.contains_key(&score) {
            scores.insert(score, vec![*number0]);
        } else {
            scores.get_mut(&score).unwrap().push(*number0);
        }
    }
    // build image
    let mut image = vec![];
    let mut rotations = vec![];
    for image0 in scores[&4].clone() {
        // rotate first tile
        let mut rotate = 0;
        while !connects.contains_key(&(image0, (rotate + 1) % 4))
            || !connects.contains_key(&(image0, (rotate + 2) % 4))
        {
            rotate += 1;
        }
        image = vec![image0.to_owned()];
        rotations = vec![rotate];
        // add all other tiles
        while image.len() < 144 {
            let y = image.len() / 12;
            let x = image.len() % 12;
            let direction;
            let image0;
            let rotate;
            if x == 0 {
                image0 = image[(y - 1) * 12 + x];
                rotate = rotations[(y - 1) * 12 + x];
                direction = 2;
            } else {
                image0 = image[y * 12 + x - 1];
                rotate = rotations[y * 12 + x - 1];
                direction = 1;
            }
            let orientation0 = (rotate + direction) % 8;
            if !connects.contains_key(&(image0, orientation0)) {
                break;
            }
            let (image1, orientation1) = connects[&(image0, orientation0)];
            //println!(
            //    "({},{}) {:?} -> {:?}",
            //    y,
            //    x,
            //    (image0, orientation0),
            //    (image1, orientation1)
            //);
            let next_rotate = (rotate + 8 - orientation0
                + orientation1
                + match orientation1 % 2 {
                    1 => 4,
                    _ => 6,
                })
                % 8;
            //print_tile(&tiles, image1, next_rotate);
            image.push(image1.to_owned());
            rotations.push(next_rotate);
        }
        if image.len() == 144 {
            break;
        }
    }
    //println!("{:?}", image);
    //println!("{:?}", rotations);
    // create bitmap
    let mut bitmap = vec![0 as i128; 96];
    for y in 0..12 {
        for x in 0..12 {
            let tile = image[y * 12 + x];
            let rotation = rotations[y * 12 + x];
            let numbers = tiles[&tile][rotation].to_owned();
            for l in 0..8 {
                let number = numbers[1+l] as i128;
                let bits = (number >> 1) & 0b11111111;
                bitmap[y * 8 + l] |= bits << (11 - x) * 8;
            }
        }
    }
    // define sea monsters
    let monsters = vec![
        vec![
            0b00000000000000000010, // ..................#.
            0b10000110000110000111, // #....##....##....###
            0b01001001001001001000, // .#..#..#..#..#..#...
        ],
        vec![
            0b01001001001001001000, // .#..#..#..#..#..#...
            0b10000110000110000111, // #....##....##....###
            0b00000000000000000010, // ..................#.
        ],
        vec![
            0b01000000000000000000, // .#..................
            0b11100001100001100001, // ###....##....##....#
            0b00010010010010010010, // ...#..#..#..#..#..#.
        ],
        vec![
            0b00010010010010010010, // ...#..#..#..#..#..#.
            0b11100001100001100001, // ###....##....##....#
            0b01000000000000000000, // .#..................
        ],
    ];
    // find monsters
    let mut result = 0;
    for m in monsters {
        let mut matches = 0;
        for y in 0..96-3 {
            for x in 0..96-20 {
                let mut lines = 0;
                for l in 0..3 {
                    let number = m[l] << (95-x);
                    if bitmap[y+l] & number == number {
                        lines+=1;
                    }
                }
                if lines == 3 {
                    matches+=1;
                }
            }
        }
        if matches>result {
            result = matches;
        }
    }
    // find roughness
    let mut ones = 0;
    for n in bitmap {
        let s = format!("{:#098b}",n);
        for c in s.to_string().chars() {
            if c=='1' {
                ones+=1;
            }
        }
    }
    println!("{}", ones - result*15);
}
