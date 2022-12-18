use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Lava {
    x: i32,
    y: i32,
    z: i32,
    sides: i32
}

fn main() {
    let lavas: HashMap<(i32, i32, i32), Lava> = read_blocks();
    // A
    let mut sa = 0;
    let min: (i32, i32, i32) = (-1, -1, -1);
    let mut max: (i32, i32, i32) = (0, 0, 0);
    for (_k, v) in lavas.iter() {
        sa += v.sides;
        if v.x > max.0 {
            max.0 = v.x;
        }
        if v.y > max.1 {
            max.1 = v.y;
        }
        if v.z > max.2 {
            max.2 = v.z;
        }
    }
    max = (max.0 + 1, max.1 + 1, max.2 + 1);
    // B
    let mut q: Vec<(i32, i32, i32)> = vec![(-1, -1, -1)];
    let mut visited: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut sa_b = 0;
    while let Some(p) = q.pop() {
        visited.insert(p);
        for i in 0..2 {
            let adj: i32 = i * 2 - 1;
            if p.0 + adj >= min.0 && p.0 + adj <= max.0 {
                if lavas.contains_key(&(p.0 + adj, p.1, p.2)) {
                    sa_b += 1;
                } else if !visited.contains(&(p.0 + adj, p.1, p.2)) {
                    q.push((p.0 + adj, p.1, p.2));
                    visited.insert((p.0 + adj, p.1, p.2));
                }
            }
            if p.1 + adj >= min.1 && p.1 + adj <= max.1 {
                if lavas.contains_key(&(p.0, p.1 + adj, p.2)) {
                    sa_b += 1;
                } else if !visited.contains(&(p.0, p.1 + adj, p.2)) {
                    q.push((p.0, p.1 + adj, p.2));
                    visited.insert((p.0, p.1 + adj, p.2));
                }
            }
            if p.2 + adj >= min.2 && p.2 + adj <= max.2 {
                if lavas.contains_key(&(p.0, p.1, p.2 + adj)) {
                    sa_b += 1;
                } else if !visited.contains(&(p.0, p.1, p.2 + adj)) {
                    q.push((p.0, p.1, p.2 + adj));
                    visited.insert((p.0, p.1, p.2 + adj));
                }
            }
        }
    }
    println!("Surface area: {}", sa);
    println!("Outer surface area: {}", sa_b);
}

fn read_blocks() -> HashMap<(i32, i32, i32), Lava> {
    let it: &str = include_str!("../input.txt");
    let mut lavas: HashMap<(i32, i32, i32), Lava> = HashMap::new();
    for l in it.lines() {
        let mut coords = l.split(",");
        let (x, y, z) = (coords.next().unwrap().parse::<i32>().unwrap(), coords.next().unwrap().parse::<i32>().unwrap(), coords.next().unwrap().parse::<i32>().unwrap());
        let mut lava: Lava = Lava { x, y, z, sides: 6 };
        for i in 0..2 {
            let adj: i32 = i * 2 - 1;
            if let Some(x_var) = lavas.get_mut(&(x + adj, y, z)) {
                lava.sides -= 1;
                x_var.sides -= 1;
            }
            if let Some(y_var) = lavas.get_mut(&(x, y + adj, z)) {
                lava.sides -= 1;
                y_var.sides -= 1;
            }
            if let Some(z_var) = lavas.get_mut(&(x, y, z + adj)) {
                lava.sides -= 1;
                z_var.sides -= 1;
            }
        }
        lavas.insert((x,y,z), lava);
    }
    lavas
}