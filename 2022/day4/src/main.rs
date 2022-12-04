use std::collections::HashSet;

fn main() {
    let input_str: &str = include_str!("../input.txt");
    let mut result_a: u32 = 0;
    let mut result_b: u32 = 0;
    for l in input_str.lines() {
        let l_spl: Vec<&str> = l.split(",").collect();
        let (mut a, mut b) = (l_spl[0].split("-"), l_spl[1].split("-"));
        let (a1, a2): (u16, u16) = (a.next().unwrap().parse::<u16>().unwrap(), a.next().unwrap().parse::<u16>().unwrap());
        let (b1, b2): (u16, u16) = (b.next().unwrap().parse::<u16>().unwrap(), b.next().unwrap().parse::<u16>().unwrap());
        let a_vec: HashSet<u16> = (a1..(a2+1)).collect();
        let b_vec: HashSet<u16> = (b1..(b2+1)).collect();
        let b_res = a_vec.intersection(&b_vec).collect::<Vec<&u16>>().into_iter().next();
        if ((a1 >= b1) & (a2 <= b2)) | ((a1 <= b1) & (a2 >= b2)) {
            result_a += 1;
        }
        match b_res {
            Some(_) => result_b += 1,
            None    => continue,
        }
    }
    println!("Result A: {}", result_a);
    println!("Result B: {}", result_b);
}
