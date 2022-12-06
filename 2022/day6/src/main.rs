use std::env;

// Run with number of indices. e.g.:
//      cargo run -- 4
fn main() {
    let args: Vec<String> = env::args().collect();
    let leng: u32 = args[1].parse().unwrap();
    let input_str: &str = include_str!("../input.txt");
    let mut ind_a: u32 = leng.clone();
    let mut vec_chars: Vec<char> = Vec::new();
    let mut found_a: bool = false;
    while !found_a {
        let str_analysis = &input_str[((ind_a - leng) as usize)..ind_a as usize];
        found_a = true;
        for c in str_analysis.chars() {
            if vec_chars.contains(&c) {
                found_a = false;
                break;
            } else {
                vec_chars.push(c);
            }
        }
        if ind_a >= input_str.len() as u32 {
            panic!("What the FUCK did you do");
        }
        if !found_a {
            ind_a += 1;
        }
        vec_chars.clear();
    }
    println!("Index for {} characters: {}", &args[1], ind_a);
}
