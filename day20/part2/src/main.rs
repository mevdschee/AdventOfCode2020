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
    // evaluate connections
    let mut connects = HashMap::new();
    for (number0, images0) in &tiles {
        for (orientation0, image0) in images0.iter().enumerate() {
            for (number1, images1) in &tiles {
                if number0 == number1 {
                    continue;
                }
                for (orientation1, image1) in images1.iter().enumerate() {
                    if image0[image0.len() - 1] == image1[0] {
                        connects.insert(
                            (*number0, orientation0),
                            (*number1, orientation1),
                        );
                    }
                }
            }
        }
    }
    // find corner tiles
    let mut scores = HashMap::new();
    for (number0, images0) in &tiles {
        let mut score = 0;
        for image0 in images0 {
            for (number1, images1) in &tiles {
                if number0 == number1 {
                    continue;
                }
                for image1 in images1 {
                    if image0[0] == image1[0] {
                        score += 1;
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
    // build image grid
    let grid_size = (tiles.len() as f64).sqrt() as usize;
    let mut placed = vec![];
    let mut orientations = vec![];
    let mut used = vec![false; tiles.len()];
    let tile_ids: Vec<i64> = tiles.keys().copied().collect();
    
    fn try_place(
        row: usize,
        col: usize,
        grid_size: usize,
        placed: &mut Vec<i64>,
        orientations: &mut Vec<usize>,
        used: &mut Vec<bool>,
        tile_ids: &Vec<i64>,
        tiles: &HashMap<i64, Vec<Vec<i32>>>,
        connects: &HashMap<(i64, usize), (i64, usize)>,
    ) -> bool {
        if row == grid_size {
            return true;
        }
        let (next_row, next_col) = if col == grid_size - 1 {
            (row + 1, 0)
        } else {
            (row, col + 1)
        };
        
        for i in 0..tile_ids.len() {
            if used[i] {
                continue;
            }
            let tile_id = tile_ids[i];
            
            for orientation in 0..8 {
                let mut fits = true;
                
                if row > 0 {
                    let above_idx = (row - 1) * grid_size + col;
                    let above_id = placed[above_idx];
                    let above_orient = orientations[above_idx];
                    if !connects.contains_key(&(above_id, above_orient)) {
                        fits = false;
                    } else {
                        let (expected_id, expected_orient) = connects[&(above_id, above_orient)];
                        if expected_id != tile_id || expected_orient != orientation {
                            fits = false;
                        }
                    }
                }
                
                if col > 0 {
                    let left_idx = row * grid_size + col - 1;
                    let left_id = placed[left_idx];
                    let left_orient = orientations[left_idx];
                    let left_image = &tiles[&left_id][left_orient];
                    let curr_image = &tiles[&tile_id][orientation];
                    
                    let mut left_edge = vec![];
                    let mut right_edge = vec![];
                    for j in 0..left_image.len() {
                        left_edge.push(left_image[j] & 1);
                        right_edge.push((curr_image[j] >> (left_image.len() - 1)) & 1);
                    }
                    if left_edge != right_edge {
                        fits = false;
                    }
                }
                
                if fits {
                    placed.push(tile_id);
                    orientations.push(orientation);
                    used[i] = true;
                    
                    if try_place(next_row, next_col, grid_size, placed, orientations, used, 
                                 tile_ids, tiles, connects) {
                        return true;
                    }
                    
                    placed.pop();
                    orientations.pop();
                    used[i] = false;
                }
            }
        }
        false
    }
    
    try_place(0, 0, grid_size, &mut placed, &mut orientations, &mut used,
              &tile_ids, &tiles, &connects);
    
    // create bitmap (96x96)
    let tile_size = 10;
    let inner_size = tile_size - 2;
    let bitmap_size = grid_size * inner_size;
    let mut bitmap = vec![0u128; bitmap_size];
    
    for y in 0..grid_size {
        for x in 0..grid_size {
            let tile_id = placed[y * grid_size + x];
            let orientation = orientations[y * grid_size + x];
            let tile_image = &tiles[&tile_id][orientation];
            
            for row in 1..tile_size - 1 {
                let bits = (tile_image[row] >> 1) & 0xFF;
                let bitmap_row = y * inner_size + row - 1;
                bitmap[bitmap_row] |= (bits as u128) << ((grid_size - 1 - x) * inner_size);
            }
        }
    }
    
    // create all orientations of bitmap
    let mut bitmaps = vec![];
    
    // normal, 90, 180, 270
    for _ in 0..4 {
        bitmaps.push(bitmap.clone());
        let mut rotated = vec![0u128; bitmap_size];
        for i in 0..bitmap_size {
            for j in 0..bitmap_size {
                let bit = (bitmap[i] >> j) & 1;
                rotated[j] |= bit << (bitmap_size - 1 - i);
            }
        }
        bitmap = rotated;
    }
    
    // flip and repeat
    bitmap = bitmaps[0].iter().rev().copied().collect();
    for _ in 0..4 {
        bitmaps.push(bitmap.clone());
        let mut rotated = vec![0u128; bitmap_size];
        for i in 0..bitmap_size {
            for j in 0..bitmap_size {
                let bit = (bitmap[i] >> j) & 1;
                rotated[j] |= bit << (bitmap_size - 1 - i);
            }
        }
        bitmap = rotated;
    }
    
    // define sea monster pattern
    let monster = vec![
        0b00000000000000000010u32,
        0b10000110000110000111u32,
        0b01001001001001001000u32,
    ];
    
    // find monsters
    let mut max_monsters = 0;
    for bitmap in &bitmaps {
        let mut matches = 0;
        for y in 0..bitmap_size - 2 {
            for x in 0..bitmap_size - 20 {
                let mut found = true;
                for (dy, pattern) in monster.iter().enumerate() {
                    let shifted = (*pattern as u128) << x;
                    if bitmap[y + dy] & shifted != shifted {
                        found = false;
                        break;
                    }
                }
                if found {
                    matches += 1;
                }
            }
        }
        if matches > max_monsters {
            max_monsters = matches;
        }
    }
    
    // count total hash marks
    let mut total = 0;
    for row in &bitmaps[0] {
        total += row.count_ones() as usize;
    }
    
    println!("{}", total - max_monsters * 15);
}
