fn main() {
    let it: &str = include_str!("../input.txt");
    let map: Vec<char> = it.lines().into_iter().flat_map(|w| w.chars().map(|h| h as char).collect::<Vec<char>>()).collect();
    let map_w: usize = it.lines().next().unwrap().len();
    let map_h: usize = it.lines().count();
    let start_raw: usize = map.iter().position(|&s| s == 'S').unwrap();
    let end_raw: usize = map.iter().position(|&e| e == 'E').unwrap();
    // ([x][y] becomes [x + y * map_h])
    let (ax, ay): (usize, usize) = (start_raw % map_w, start_raw / map_w);
    let (bx, by): (usize, usize) = (end_raw % map_w, end_raw / map_w);

    // Breadth first search implementation
    let steps_a: u32 = bfs(&map, map_w, map_h, ax, ay, 'E');
    let steps_b: u32 = bfs(&map, map_w, map_h, bx, by, 'a');

    println!("Start: {} ({}, {}), End: {} ({}, {})", start_raw, ax, ay, end_raw, bx, by);
    println!("Steps: {}", steps_a);
    println!("Scenic Route: {}", steps_b);
}

fn bfs(map: &Vec<char>, map_w: usize, map_h: usize, ax: usize, ay: usize, dest_char: char) -> u32 {
    let mut bfs_queue: Vec<(usize, usize, u32)> = Vec::from([(ax, ay, 0)]);
    let mut bfs_visited: Vec<(usize, usize)> = Vec::new();
    let mut steps: u32 = 0;
    let reverse: bool = dest_char != 'E';
    while let Some((currx, curry, currs)) = bfs_queue.pop() {
        if bfs_visited.iter().any(|&x| x == (currx, curry)) {
            continue;
        }
        if map[currx + curry * map_w] == dest_char {
            steps = currs;
            break;
        }
        bfs_visited.push((currx, curry));
        if curry != map_h - 1 {
            if bfs_check(&map, currx + curry * map_w, currx + (curry + 1) * map_w, reverse) {
                // println!("Iteration {}: {} ({}, [{}, {}]) to {} ({})", currs, map[(currx + curry * map_w)], (currx + curry * map_w), currx, curry, map[(currx + (curry + 1) * map_w)], (currx + (curry + 1) * map_w));
                bfs_queue.insert(0, (currx, curry + 1, currs + 1));
            }
        }
        if curry != 0 {
            if bfs_check(&map, currx + curry * map_w, currx + (curry - 1) * map_w, reverse) {
                // println!("Iteration {}: {} ({}, [{}, {}]) to {} ({})", currs, map[(currx + curry * map_w)], (currx + curry * map_w), currx, curry, map[(currx + (curry - 1) * map_w)], (currx + (curry - 1) * map_w));
                bfs_queue.insert(0, (currx, curry - 1, currs + 1));
            }
        }
        if currx != map_w - 1 {
            if bfs_check(&map, currx + curry * map_w, (currx + 1) + curry * map_w, reverse) {
                // println!("Iteration {}: {} ({}, [{}, {}]) to {} ({})", currs, map[(currx + curry * map_w)], (currx + curry * map_w), currx, curry, map[((currx + 1) + curry * map_w)], ((currx + 1) + curry * map_w));
                bfs_queue.insert(0, (currx + 1, curry, currs + 1));
            }
        }
        if currx != 0 {
            if bfs_check(&map, currx + curry * map_w, (currx - 1) + curry * map_w, reverse) {
                // println!("Iteration {}: {} ({}, [{}, {}]) to {} ({})", currs, map[(currx + curry * map_w)], (currx + curry * map_w), currx, curry, map[((currx - 1) + curry * map_w)], ((currx - 1) + curry * map_w));
                bfs_queue.insert(0, (currx - 1, curry, currs + 1));
            }
        }
    }
    return steps;
}

fn bfs_check(map: &Vec<char>, a: usize, b: usize, reverse: bool) -> bool {
    if !reverse {
        let diff: i32 = map[b] as i32 - map[a] as i32;
        return ((diff <= 1) & ((map[b] != 'E') | (map[a] < 'y'))) | (map[a] == 'S');
    } else {
        let diff: i32 = map[a] as i32 - map[b] as i32;
        return ((diff <= 1) & ((map[b] != 'a') | (map[a] >= 'b'))) | (map[a] == 'E');
    }
}