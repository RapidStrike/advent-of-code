use std::env;

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    op: char,
    factor: i64,
    test_factor: i64,
    mon_true: i64,
    mon_false: i64
}

// Run:
// cargo run -- {loop count} {sanity}
fn main() {
    let mut args = env::args();
    args.next();

    let mut monkeys: Vec<Monkey> = Vec::new();
    let rounds: u32 = args.next().unwrap().parse().unwrap();
    let sanity: bool = args.next().unwrap().parse().unwrap();
    let mut max_loop: i64 = 1;
    start_parse(&mut monkeys, &mut max_loop);
    println!("{}", max_loop);
    let mut monkey_counts: Vec<u64> = vec![0; monkeys.len()];
    for _i in 0..rounds {
        for m in 0..monkeys.len() {
            let clone = monkeys[m].items.clone();
            let mut item_iter = clone.iter();
            while let Some(item) = item_iter.next() {
                let mut item_mut = *item;
                if monkeys[m].op == '+' {
                    item_mut += if monkeys[m].factor > 0 { monkeys[m].factor } else { item_mut };
                } else {
                    item_mut *= if monkeys[m].factor > 0 { monkeys[m].factor } else { item_mut };
                }
                // println!("{} {} {} = {}", item, monkeys[m].op, monkeys[m].factor, item_mut);
                if sanity {
                    item_mut /= 3;
                }
                item_mut = item_mut.rem_euclid(max_loop);
                // println!("/3 -> {}", item_mut);
                if item_mut % monkeys[m].test_factor == 0 {
                    let t = monkeys[m].mon_true;
                    monkeys[usize::try_from(t).unwrap()].items.push(item_mut);
                    // println!("{} goes to Monkey {}", item_mut, t);
                } else {
                    let f = monkeys[m].mon_false;
                    monkeys[usize::try_from(f).unwrap()].items.push(item_mut);
                    // println!("{} goes to Monkey {}", item_mut, f);
                }
                monkey_counts[m] += 1;
            }
            monkeys[m].items.clear();
            // println!("-------------------------");
        }
    }
    monkey_counts.sort();
    println!("Total monkeys: {:?}", monkey_counts);
    println!("Monkey business: {}", monkey_counts[monkey_counts.len() - 2] * monkey_counts[monkey_counts.len() - 1]);
}

fn start_parse(monkeys: &mut Vec<Monkey>, max_loop: &mut i64) {
    let it: &str = include_str!("../input.txt");
    let mut n_items: Vec<i64> = Vec::new();
    let mut n_op: char = ' ';
    let mut n_factor: i64 = -2;
    let mut n_test_factor: i64 = -2;
    let mut n_mon_true: i64 = -2;
    let mut n_mon_false: i64 = -2;
    for l in it.lines() {
        let mut l_parse = l.split(":");
        match l_parse.next().unwrap().trim() {
            "Starting items" => {
                // :)
                n_items = l_parse.next().unwrap().trim().split(",").map(|i| i.trim().parse::<i64>().unwrap()).collect();
            },
            "Operation" => {
                let mut op_parse = l_parse.next().unwrap().trim().split(" ");
                n_op = op_parse.nth(3).unwrap().parse().unwrap();
                let f_test = op_parse.next().unwrap().parse();
                match f_test {
                    Ok(x) => n_factor = x,
                    Err(_) => n_factor = -1
                }
            },
            "Test" => {
                n_test_factor = l_parse.next().unwrap().trim().split(" ").nth(2).unwrap().parse::<i64>().unwrap();
                *max_loop *= n_test_factor;
            },
            "If true" => {
                n_mon_true = l_parse.next().unwrap().trim().split(" ").nth(3).unwrap().parse::<i64>().unwrap();
            },
            "If false" => {
                n_mon_false = l_parse.next().unwrap().trim().split(" ").nth(3).unwrap().parse::<i64>().unwrap();
            },
            "" => {
                // Monkey finished.
                monkeys.push(Monkey {
                    items: n_items.clone(),
                    op: n_op,
                    factor: n_factor,
                    test_factor: n_test_factor,
                    mon_true: n_mon_true,
                    mon_false: n_mon_false
                });
            },
            _ => ()
        }
    }
    monkeys.push(Monkey {
        items: n_items.clone(),
        op: n_op,
        factor: n_factor,
        test_factor: n_test_factor,
        mon_true: n_mon_true,
        mon_false: n_mon_false
    });
}
