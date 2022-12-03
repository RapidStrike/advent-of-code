use std::collections::HashSet;

fn main() {
    let input_str: &str = include_str!("../input.txt");
    let mut total_a: u32 = 0;
    let mut total_b: u32 = 0;
    let mut curr_b: HashSet<char> = HashSet::new();
    let mut iter: u16 = 0;
    for l in input_str.lines() {
        let s1: HashSet<char> = l[..(l.len()/2)].chars().collect();
        let s2: HashSet<char> = l[(l.len()/2)..].chars().collect();
        if iter % 3 == 0 {
            curr_b = l.chars().collect();
        } else {
            curr_b = curr_b.clone().intersection(&l.chars().collect()).into_iter().map(|c| *c).collect();
        }
        let sm: char = *s1.intersection(&s2).collect::<Vec<&char>>().into_iter().next().unwrap();
        let sm_v: u8 = if sm >= 'a' {(sm as u8) - 96} else {(sm as u8) - 38};
        if iter % 3 == 2 {
            let sm_b: char = curr_b.clone().into_iter().next().unwrap();
            let sm_b_v: u8 = if sm_b >= 'a' {(sm_b as u8) - 96} else {(sm_b as u8) - 38};
            total_b += sm_b_v as u32;
            println!("Iteration {}: {} -> {}", iter, sm, sm_v);
        }
        total_a += sm_v as u32;
        iter += 1;
        // println!("{}: {} = {}", l, sm, sm_v);
    }
    println!("Priority total A: {}", total_a);
    println!("Badge total B: {}", total_b);
}
