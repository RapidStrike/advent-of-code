fn main() {
    let it: &str = include_str!("../input.txt");
    let orig: Vec<i64> = it.lines().map(|c| c.parse::<i64>().unwrap()).collect();
    // A
    let mut coords: Vec<(usize, i64)> = orig.clone().iter().enumerate().map(|(i, n)| (i, *n)).collect();
    for (i, _) in orig.iter().enumerate() {
        coords = set_index(&coords, i);
    }
    let mut zero_coord = coords.iter().position(|(_, n)| *n == 0).unwrap();
    let (mut x, mut y, mut z) = ((zero_coord + 1000) % orig.len(), (zero_coord + 2000) % orig.len(), (zero_coord + 3000) % orig.len());
    // println!("Zero pos: {}", zero_coord);
    println!("Coordinate Sum (A): {}", coords[x].1 + coords[y].1 + coords[z].1);

    // B
    coords = orig.clone().iter().enumerate().map(|(i, n)| (i, *n * 811589153i64)).collect();
    for _ in 0..10 {
        for (i, _) in orig.iter().enumerate() {
            coords = set_index(&coords, i);
        }
    }
    zero_coord = coords.iter().position(|(_, n)| *n == 0).unwrap();
    (x, y, z) = ((zero_coord + 1000) % orig.len(), (zero_coord + 2000) % orig.len(), (zero_coord + 3000) % orig.len());
    // println!("Zero pos: {}", zero_coord);
    println!("Coordinate Sum (B): {}", coords[x].1 + coords[y].1 + coords[z].1);
}

fn set_index(coords: &Vec<(usize, i64)>, index: usize) -> Vec<(usize, i64)> {
    let mut new_coords: Vec<(usize, i64)> = coords.clone();
    let mut item_loc: usize = coords.iter().position(|(i, _)| *i == index).unwrap();
    let item: (usize, i64) = new_coords.remove(item_loc);
    let mut loc_raw: i64 = item_loc as i64 + item.1 + ((item_loc as i64 + item.1) / (coords.len() as i64 - 1));
    if item_loc as i64 + item.1 <= 0 {
        // Just one, since ((item_loc_n + item.1) / coords.len() as i64) <= 0
        loc_raw -= 1;
    }
    item_loc = ((loc_raw + (coords.len() as i64 * 811589153i64 * 3i64)) % coords.len() as i64) as usize;
    new_coords.insert(item_loc, item);
    return new_coords;
}