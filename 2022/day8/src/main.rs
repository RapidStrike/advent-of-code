fn main() {
    let it: &str = include_str!("../input.txt");
    let rows: usize = it.lines().count();
    let cols: usize = it.lines().next().unwrap().len();
    let forest: Vec<u32> = it.lines().into_iter().map(|w| w.chars().map(|h| h.to_digit(10).unwrap()).collect::<Vec<u32>>()).flatten().collect();
    let mut exposed_a: u32 = 0;
    let mut max_scene_b: u32 = 0;
    // read table ([w][h] becomes [w + h * rows])
    for h in 0..rows {
        for w in 0..cols {
            let curr = &forest[w + h * rows];
            if (h == 0) | (w == 0) | (h == rows - 1) | (w == cols - 1) {
                exposed_a += 1;
            } else {
                let mut north_iter = (0..h).into_iter().map(|n| &forest[w + n * rows]).rev();
                let mut south_iter = ((h + 1)..rows).into_iter().map(|n| &forest[w + n * rows]);
                let mut west_iter = forest[(h * rows)..(w + h * rows)].into_iter().rev();
                let mut east_iter = forest[(w + 1 + h * rows)..(cols + h * rows)].into_iter();
                // A
                let north_h = north_iter.clone().max().unwrap();
                let south_h = south_iter.clone().max().unwrap();
                let west_h = west_iter.clone().max().unwrap();
                let east_h = east_iter.clone().max().unwrap();
                if (north_h < curr) | (south_h < curr) | (west_h < curr) | (east_h < curr) {
                    // println!("{} x {}: {} (N={}, S={}, W={}, E={})", w, h, curr, north_h, south_h, west_h, east_h);
                    exposed_a += 1;
                }
                // B
                // >if_only_there_were_an_easier_way_to_do_this.jpg
                let mut n_score = 0;
                let mut s_score = 0;
                let mut e_score = 0;
                let mut w_score = 0;
                while let Some(n) = north_iter.next() {
                    n_score += 1;
                    if n >= curr {
                        break;
                    }
                }
                while let Some(s) = south_iter.next() {
                    s_score += 1;
                    if s >= curr {
                        break;
                    }
                }
                while let Some(w) = west_iter.next() {
                    w_score += 1;
                    if w >= curr {
                        break;
                    }
                }
                while let Some(e) = east_iter.next() {
                    e_score += 1;
                    if e >= curr {
                        break;
                    }
                }
                let scene_score = n_score * s_score * e_score * w_score;
                if scene_score > max_scene_b {
                    max_scene_b = scene_score;
                }
            }
        }
    }
    println!("Exposed trees (A): {}", exposed_a);
    println!("Max scenery score (B): {}", max_scene_b);
}
