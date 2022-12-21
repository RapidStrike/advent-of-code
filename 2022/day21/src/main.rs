use std::collections::HashMap;

#[derive(Debug)]
struct Monkey {
    ref_a: String,
    ref_b: String,
    op: char,
    answer: Option<i64>
}

fn main() {
    let it: &str = include_str!("../input.txt");
    let mut monkeys: HashMap<String, Monkey> = HashMap::new();
    for l in it.lines() {
        let objs: Vec<String> = l.split(": ").map(|s| s.to_string()).collect();
        if is_str_numeric(&objs[1]) {
            monkeys.insert(objs[0].clone(), Monkey { ref_a: "a".to_string(), ref_b: "b".to_string(), op: '+', answer: Some(objs[1].parse::<i64>().unwrap()) });
        } else {
            let (ref_a, op, ref_b) = (objs[1][0..4].to_string(), objs[1][5..6].chars().next().unwrap(), objs[1][7..].to_string());
            monkeys.insert(objs[0].clone(), Monkey { ref_a, ref_b, op, answer: None });
        }
    }
    println!("Root answer (A): {}", calculate_monkey(&monkeys, &"root".to_string()));
    // B
    let ref_a_has_humn = has_elem(&monkeys, &monkeys["root"].ref_a, &"humn".to_string());
    let mnky_nm = if ref_a_has_humn { &monkeys["root"].ref_a } else { &monkeys["root"].ref_b };
    let mnky_val = calculate_monkey(&monkeys, if ref_a_has_humn { &monkeys["root"].ref_b } else { &monkeys["root"].ref_a });
    println!("Human answer (B): {}", calculate_monkey_rev(&monkeys, mnky_nm, mnky_val));
}

fn calculate_monkey(monkeys: &HashMap<String, Monkey>, name: &String) -> i64 {
    match monkeys[name].answer {
        Some(x) => { return x },
        None => {
            let monkey_a = calculate_monkey(&monkeys, &monkeys[name].ref_a);
            let monkey_b = calculate_monkey(&monkeys, &monkeys[name].ref_b);
            match monkeys[name].op {
                '+' => { return monkey_a + monkey_b; },
                '-' => { return monkey_a - monkey_b; },
                '*' => { return monkey_a * monkey_b; },
                '/' => { return monkey_a / monkey_b; },
                _ => { unreachable!("What the fuck happened!"); }
            }
        }
    }
}

fn calculate_monkey_rev(monkeys: &HashMap<String, Monkey>, curr_name: &String, curr_val: i64) -> i64 {
    if curr_name == "humn" {
        return curr_val;
    }
    match monkeys[curr_name].answer {
        Some(_) => { unreachable!("This should not happen!"); },
        None => {
            let ref_a_has_humn = has_elem(&monkeys, &monkeys[curr_name].ref_a, &"humn".to_string());
            let mnky_nm = if ref_a_has_humn { &monkeys[curr_name].ref_a } else { &monkeys[curr_name].ref_b };
            let non_humn_val = calculate_monkey(&monkeys, if ref_a_has_humn { &monkeys[curr_name].ref_b } else { &monkeys[curr_name].ref_a });
            match monkeys[curr_name].op {
                '+' => { 
                    return calculate_monkey_rev(&monkeys, &mnky_nm, curr_val - non_humn_val);
                },
                '-' => {
                    // val = a - b
                    if ref_a_has_humn {
                        return calculate_monkey_rev(&monkeys, &mnky_nm, curr_val + non_humn_val);
                    } else {
                        return calculate_monkey_rev(&monkeys, &mnky_nm, non_humn_val - curr_val);
                    }
                },
                '*' => {
                    return calculate_monkey_rev(&monkeys, &mnky_nm, curr_val / non_humn_val);
                },
                '/' => {
                    // val = a / b
                    if ref_a_has_humn {
                        return calculate_monkey_rev(&monkeys, &mnky_nm, curr_val * non_humn_val);
                    } else {
                        return calculate_monkey_rev(&monkeys, &mnky_nm, non_humn_val / curr_val);
                    }
                },
                _ => { unreachable!("What the fuck!!!"); }
            }
        }
    }
}

fn has_elem(monkeys: &HashMap<String, Monkey>, parent: &String, elem: &String) -> bool {
    if parent == elem {
        return true;
    } else if monkeys[parent].answer == None {
        if has_elem(&monkeys, &monkeys[parent].ref_a, &elem) {
            return true;
        } else if has_elem(&monkeys, &monkeys[parent].ref_b, &elem) {
            return true;
        }
    }
    return false;
}

fn is_str_numeric(s: &String) -> bool {
    for i in s.chars() {
        if !i.is_numeric() {
            return false;
        }
    }
    return true;
}