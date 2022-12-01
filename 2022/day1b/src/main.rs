fn main() {
    let input_str: &str             = include_str!("../input.txt");
    let mut elf_vector: Vec<u32>    = Vec::new();
    let mut curr_elf_cals: u32      = 0;
    for line in input_str.lines() {
        if line != "" {
            curr_elf_cals = curr_elf_cals + line.parse::<u32>().unwrap();
        } else {
            elf_vector.push(curr_elf_cals);
            curr_elf_cals = 0;
        }
    }
    elf_vector.sort();
    elf_vector.reverse();
    curr_elf_cals = &elf_vector[0] + &elf_vector[1] + &elf_vector[2];
    println!("{}", curr_elf_cals);
}