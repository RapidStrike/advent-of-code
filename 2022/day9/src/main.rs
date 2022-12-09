use std::collections::HashSet;
use std::env;

// Run:
// cargo run -- {rope_length}
fn main() {
    let it: &str = include_str!("../input.txt");
    let rope_length: u32 = env::args().last().unwrap().parse().unwrap();
    let mut stepped: HashSet<(i32, i32)> = HashSet::new();
    let mut r: Vec<(i32, i32)> = vec![(0, 0); rope_length as usize];
    for l in it.lines() {
        let mut vars = l.split(" ");
        let dir = vars.next().unwrap();
        let step: u32 = vars.next().unwrap().parse().unwrap();
        
        for _i in 0..step {
            let (mut k1x, mut k1y) = r[0];
            match dir {
                "U" => k1y += 1,
                "D" => k1y -= 1,
                "R" => k1x += 1,
                _   => k1x -= 1
            }
            r[0] = (k1x, k1y);
            update_knots(&mut r);
            stepped.insert(r[r.len() - 1]);
        }
    }
    println!("Total amount of unique steps: {}", stepped.len());
    println!("{:?}", r);
}

fn update_knots(r: &mut Vec<(i32, i32)>) {
    for i in 1..r.len() {
        let (k1x, k1y) = r[i-1];
        let (mut k2x, mut k2y) = r[i];
        let (diff_x, diff_y) = (k1x - k2x, k1y - k2y);
        if (diff_x == 0) | (diff_y == 0) {
            // non-diag
            if diff_x > 1 {
                k2x += 1;
            } else if diff_x < -1 {
                k2x -= 1;
            } else if diff_y > 1 {
                k2y += 1;
            } else if diff_y < -1 {
                k2y -= 1;
            }
        } else if ((diff_x > 1) | (diff_x < -1)) & ((diff_y > 1) | (diff_y < -1)) {
            // super-diag!!
            if diff_x > 1 {
                k2x += 1;
            } else {
                k2x -= 1;
            }
            if diff_y > 1 {
                k2y += 1;
            } else {
                k2y -= 1;
            }
        } else {
            // diag
            if diff_x > 1 {
                k2y = k1y;
                k2x += 1;
            } else if diff_x < -1 {
                k2y = k1y;
                k2x -= 1;
            } else if diff_y > 1 {
                k2x = k1x;
                k2y += 1;
            } else if diff_y < -1 {
                k2x = k1x;
                k2y -= 1;
            }
        }
        r[i] = (k2x, k2y);
    }
}
