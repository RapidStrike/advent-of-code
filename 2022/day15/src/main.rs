use regex::Regex;

fn main() {
    let y_line = 2000000;
    let it: &str = include_str!("../input.txt");
    let mut h_lines: Vec<(i32, i32)> = Vec::new();
    let mut diamonds: Vec<(i32, i32, i32)> = Vec::new();
    let mut detected_pts: i32 = 0;
    // tfw lazy
    let re_pts = Regex::new(r".*x=(-?\d+), y=(-?\d+).*x=(-?\d+), y=(-?\d+)").unwrap();
    for l in it.lines() {
        let pts_cap = re_pts.captures(l).unwrap();
        let (x1, y1) = (&pts_cap[1].parse::<i32>().unwrap(), &pts_cap[2].parse::<i32>().unwrap());
        let (x2, y2) = (&pts_cap[3].parse::<i32>().unwrap(), &pts_cap[4].parse::<i32>().unwrap());
        let dist: i32 = (x1 - x2).abs() + (y1 - y2).abs();
        diamonds.push((*x1, *y1, dist));
        let y_dist = (y1 - y_line).abs();
        if y_dist <= dist {
            h_lines.push((x1 - (dist - y_dist).abs(), x1 + (dist - y_dist).abs()));
            // *y += 2 * (dist - y_dist) + 1;
        }
        // println!("Distance: {}\nStart: ({}, {})\nDistance to line: {}\n------------", dist, x1, y1, y_dist);
    }
    // A
    if h_lines.len() > 1 {
        h_lines.sort();
        let (mut x1, mut x2) = h_lines[0];
        for x in 1..h_lines.len() {
            let (x3, x4) = h_lines[x];
            if x3 <= x2 {
                if x4 <= x2 {
                    continue;
                } else {
                    x2 = x4;
                }
            } else {
                detected_pts += x2 - x1;
                (x1, x2) = (x3, x4);
            }
        }
        detected_pts += x2 - x1;
    }
    println!("Visible points (A): {}", detected_pts);
    // B
    let mut segs: Vec<(i32, i32)> = Vec::new();
    for y in 0..=4000000 {
        h_lines.clear();
        segs.clear();
        for (x1, y1, dist) in &diamonds {
            let y_dist = (y1 - y).abs();
            if y_dist <= *dist {
                h_lines.push((x1 - (dist - y_dist).abs(), x1 + (dist - y_dist).abs()));
            }
        }
        if h_lines.len() > 1 {
            h_lines.sort();
            let (mut x1, mut x2) = h_lines[0];
            for x in 1..h_lines.len() {
                let (x3, x4) = h_lines[x];
                if x3 <= x2 {
                    if x4 <= x2 {
                        continue;
                    } else {
                        x2 = x4;
                    }
                } else {
                    segs.push((x1, x2));
                    (x1, x2) = (x3, x4);
                }
            }
            segs.push((x1, x2));
            if segs.len() > 1 {
                // Found!
                let x = (segs[0].1 + segs[1].0) / 2;
                println!("({}, {})", x, y);
                println!("Frequency (B): {}{}", x * 4 + (y / 1000000), y % 1000000);
            }
        }
    }
}