fn main() {
    let input_str: &str         = include_str!("../input.txt");
    let mut curr_elf_cals: u32  = 0;
    let mut max_elf_cals: u32   = 0;
    for line in input_str.lines() {
        if line != "" {
            curr_elf_cals = curr_elf_cals + line.parse::<u32>().unwrap();
        } else {
            if max_elf_cals < curr_elf_cals {
                max_elf_cals = curr_elf_cals;
            }
            curr_elf_cals = 0;
        }
    }
    println!("{}", max_elf_cals.to_string());
}