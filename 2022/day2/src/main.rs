fn main() {
    let input_str: &str = include_str!("../input.txt");
    let mut curr_score_a: i32 = 0;
    let mut curr_score_b: i32 = 0;
    for line in input_str.lines() {
        let cl: Vec<&str> = line.split(" ").collect();
        let a_a: i32 = if cl[1] == "X" { 1 } else if cl[1] == "Y" { 2 } else { 3 };
        let b: i32 = if cl[0] == "A" { 1 } else if cl[0] == "B" { 2 } else { 3 };
        // a
        let ab: i32 = a_a - b;
        let win_a: i32 = if ab == 0 { 3 } else if (ab == 1) | (ab == -2) { 6 } else { 0 };
        // b
        let choice: i32 = a_a - 2;
        let a_b: i32 = (b + choice - 1).rem_euclid(3) + 1;
        let win_b: i32 = if a_a == 1 { 0 } else if a_a == 2 { 3 } else { 6 };
        curr_score_a = curr_score_a + a_a + win_a;
        curr_score_b = curr_score_b + a_b + win_b;
    }
    println!("Final score A: {}", curr_score_a);
    println!("Final score B: {}", curr_score_b);
}
