use regex::Regex;

fn main() {
    let input_str: &str = include_str!("../input.txt");
    let mut stacks_a: Vec<Vec<char>> = init_stacks(input_str);
    let mut stacks_b: Vec<Vec<char>> = stacks_a.clone();
    let re_step = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for l in input_str.lines() {
        if re_step.is_match(l) {
            let cap = re_step.captures(l).unwrap();
            let a: usize = cap.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
            let b: usize = cap.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;
            let ct: u16 = cap.get(1).unwrap().as_str().parse().unwrap();
            
            let mut mv_b: Vec<char> = stacks_b[a][(stacks_b[a].len() - (ct as usize))..stacks_b[a].len()].to_vec();
            for _ in 0..ct {
                let mv = stacks_a[a].pop().unwrap();
                stacks_b[a].pop();
                stacks_a[b].push(mv);
            }
            stacks_b[b].append(& mut mv_b);
        }
    }
    let res_a: Vec<char> = stacks_a.iter().map(|x| x[x.len() - 1]).collect();
    let res_b: Vec<char> = stacks_b.iter().map(|x| x[x.len() - 1]).collect();
    println!("Result A: {:?}", res_a.into_iter().collect::<String>());
    println!("Result B: {:?}", res_b.into_iter().collect::<String>());
}

fn init_stacks(i: &str) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..9 {
        stacks.push(Vec::new());
    }
    let re_box = Regex::new(r"(?:   |\[(.)\]) ?").unwrap();
    let mut cap_iter = 0;
    for l in i.lines() {
        for capt in re_box.captures_iter(l) {
            match capt.get(1) {
                Some(x) => stacks[cap_iter].push(x.as_str().chars().next().unwrap()),
                None => (),
            }
            cap_iter += 1;
        }
        cap_iter = 0;
    }
    for i in 0..9 {
        stacks[i].reverse();
    }
    // This is what I was GONNA use but then I was like "no you gotta do this shit properly" so
    // stacks.push(vec!['B', 'L', 'D', 'T', 'W', 'C', 'F', 'M']);
    // stacks.push(vec!['N', 'B', 'L']);
    // stacks.push(vec!['J', 'C', 'H', 'T', 'L', 'V']);
    // stacks.push(vec!['S', 'P', 'J', 'W']);
    // stacks.push(vec!['Z', 'S', 'C', 'F', 'T', 'L', 'R']);
    // stacks.push(vec!['W', 'D', 'G', 'B', 'H', 'N', 'Z']);
    // stacks.push(vec!['F', 'M', 'S', 'P', 'V', 'G', 'C', 'N']);
    // stacks.push(vec!['W', 'Q', 'R', 'J', 'F', 'V', 'C', 'Z']);
    // stacks.push(vec!['R', 'P', 'M', 'L', 'H']);
    return stacks;
}