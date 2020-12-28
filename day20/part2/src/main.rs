use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn print_tile(tiles: &HashMap<i32, Vec<Vec<i32>>>, tile: i32, rotation: usize) {
    println!("{} {}", tile,rotation);
    for n in &tiles[&tile][rotation] {
        let s = format!("{:#012b}", n);
        println!("{}",s[2..].to_string().replace("0","..").replace("1","()"));
    }
}

fn tile_connects(tiles: &HashMap<i32, Vec<Vec<i32>>>, tile0: i32, tile1: i32) -> bool {
    let mut found = false;
    for side0 in &tiles[&tile0] {
        for side1 in &tiles[&tile1] {
            if side0[0 as usize] == side1[0 as usize] {
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }
    return found;
}

fn find_next_tile(
    tiles: &HashMap<i32, Vec<Vec<i32>>>,
    image: &Vec<i32>,
    scores: &HashMap<i32, Vec<i32>>,
) -> i32 {
    let y = image.len() / 12;
    let x = image.len() % 12;
    let mut pool = 4;
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
        if y == 0 || tile_connects(tiles, *tile, image[(y - 1) * 12 + x]) {
            if x == 0 || tile_connects(tiles, *tile, image[y * 12 + x - 1]) {
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
        for (orientation0,image0) in images0.iter().enumerate() {
            for (number1, images1) in &tiles {
                if number0 == number1 {
                    continue;
                }
                for (orientation1,image1) in images1.iter().enumerate() {
                    if image0[0] == image1[0] {
                        score += 1;
                        connects.insert((number0,orientation0),(number1,orientation1));
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
    //println!("{:?}",connects);
    let image0 = &scores[&4][0];
    // first tile orientation
    let mut rotate = 0;
    while !connects.contains_key(&(image0,(rotate+1)%4)) || !connects.contains_key(&(image0,(rotate+2)%4)) {
        rotate+=1;
    }
    // build image
    let mut image = vec![image0];
    let mut rotations = vec![rotate];
    print_tile(&tiles, *image0, rotate);

    //for y in 0..12 {
    //    for x in 0..12 {
            let orientation0 = (rotate+1)%4;
            let (image1,orientation1) = connects[&(image0,orientation0)];
            println!("({},{}) {:?} -> {:?}",0,1,(image0,orientation0),(image1,orientation1));
            /*let mut next_rotate = rotate + match (orientation0, orientation1) {
                (0,4) => 6,   // 1 = (3 + (8 - 0) + 4 + 6) % 8
                (1,2) => 7,
                (1,0) => 5,
                (1,7) => 2,
                _ => {
                    print_tile(&tiles, *image0, rotate);
                    print_tile(&tiles, *image1, rotate);
                    panic!("Unknown combination: {:?}", (orientation0, orientation1))
                },
            };*/
            let mut next_rotate = rotate + ( 8 - orientation0) + orientation1 + 6;
            //if ((8 - orientation0) + orientation1)%2==0 {
            //    next_rotate += 8;
            //} else {
            //    next_rotate += 6;
            //}
            next_rotate %= 8;
            print_tile(&tiles, *image0, rotate);
            print_tile(&tiles, *image1, next_rotate);
//        }
//    }
    //    image.push(find_next_tile(&tiles, &image, &connects));
    //}
    /*for i in 0..2 {
        println!("{}", image[i]);
        print_tile(&tiles, image[i], 0);
    }*/
    //println!("{:?}", image);
}
