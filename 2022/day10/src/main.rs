fn main() {
    let it: &str = include_str!("../input.txt");
    let mut rx: i32 = 1;
    let mut cycle: i32 = 0;
    let mut total: i32 = 0;
    let mut crt: String = String::from("");
    for l in it.lines() {
        let mut l_spl = l.split(" ");
        let instr = l_spl.next().unwrap();
        cycle_check(&mut cycle, &mut rx, &mut total, &mut crt);
        if instr == "addx" {
            let addx: i32 = l_spl.next().unwrap().parse().unwrap();
            cycle_check(&mut cycle, &mut rx, &mut total, &mut crt);
            rx += addx;
        }
    }
    println!("Signal strength total (A): {}", total);
    println!("{}", crt);
}

fn cycle_check(cycle: &mut i32, rx: &mut i32, total: &mut i32, crt: &mut String) {
    *cycle += 1;
    let cyc_check = cycle.rem_euclid(40);
    if cyc_check == 20 {
        *total += *cycle * *rx;
        // println!("{}", total);
    }
    let c: char = if i32::abs(*rx - cyc_check + 1) <= 1 {'#'} else {' '};
    crt.push(c);
    // next row
    if cyc_check == 0 {
        crt.push('\n');
    }
}
