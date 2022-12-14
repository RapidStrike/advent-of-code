fn main() {
    let mut map: Vec<Vec<u8>> = vec![vec![0; 200]; 1000];
    let bottom_pos: usize = draw_map(&mut map);
    let sand_count_a: u32 = sand_game(&mut map, false, bottom_pos);
    let sand_count_b: u32 = sand_game(&mut map, true, bottom_pos) + sand_count_a;
    
    println!("Sand Units (A): {}", sand_count_a);
    println!("Sand Units (B): {}", sand_count_b);
}

fn sand_game(map: &mut Vec<Vec<u8>>, bottom: bool, bottom_pos: usize) -> u32 {
    let mut sand_count: u32 = 0;
    let mut stop: bool = false;
    while !stop {
        let (mut x, mut y) = (500 as usize, 0 as usize);
        // Fall
        loop {
            if map[x][y + 1] == 0 {
                y += 1;
            } else if map[x - 1][y + 1] == 0 {
                y += 1;
                x -= 1;
            } else if map[x + 1][y + 1] == 0 {
                y += 1;
                x += 1;
            } else {
                map[x][y] = 2;
                sand_count += 1;
                if (x, y) == (500, 0) {
                    stop = true;
                }
                break;
            }
            if (!bottom) & (y == bottom_pos - 1) {
                stop = true;
                break;
            } else if y == bottom_pos - 1 {
                map[x][y] = 2;
                sand_count += 1;
                break;
            }
        }
    }
    return sand_count;
}

fn draw_map(map: &mut Vec<Vec<u8>>) -> usize {
    let it: &str = include_str!("../input.txt");
    let mut bottom: usize = 0;
    for l in it.lines() {
        let mut raw_coords = l.split(" -> ");
        let mut coords: Vec<(i32, i32)> = Vec::new();
        while let Some(rc) = raw_coords.next() {
            let mut coord_i = rc.split(",");
            let (c_x, c_y) = (coord_i.next().unwrap().parse::<i32>().unwrap(), coord_i.next().unwrap().parse::<i32>().unwrap());
            coords.push((c_x, c_y));
            if c_y as usize > bottom {
                bottom = c_y as usize;
            }
        }
        for i in 1..coords.len() {
            // From 2nd point to 1st point
            let x_diff = coords[i].0 - coords[i-1].0;
            let y_diff = coords[i].1 - coords[i-1].1;
            match (x_diff, y_diff) {
                (x, _) if x < 0 => {
                    // Left
                    for d in 0..=(x_diff.abs() as usize) {
                        map[coords[i].0 as usize + d][coords[i].1 as usize] = 1;
                    }
                },
                (x, _) if x > 0 => {
                    // Right
                    for d in 0..=(x_diff as usize) {
                        map[coords[i].0 as usize - d][coords[i].1 as usize] = 1;
                    }
                },
                (_, y) if y < 0 => {
                    // Up
                    for d in 0..=(y_diff.abs() as usize) {
                        map[coords[i].0 as usize][coords[i].1 as usize + d] = 1;
                    }
                },
                (_, y) if y > 0 => {
                    // Down
                    for d in 0..=(y_diff as usize) {
                        map[coords[i].0 as usize][coords[i].1 as usize - d] = 1;
                    }
                },
                _ => ()
            }
        }
    }
    for i in 0..map[0].len() {
        for j in 400..600 {
            print!("{}", if (j == 500) & (i == 0) {'V'} else if map[j][i] == 0 {'.'} else {'#'});
        }
        println!("");
    }
    return bottom + 2;
}