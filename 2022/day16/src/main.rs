use std::cmp::Ordering;
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
struct Valve {
    flow: i32,
    next: Vec<String>
}

#[derive(Debug, Eq, PartialEq)]
struct Instance {
    curr: String,
    e_curr: String,
    flow_rate: i32,
    open_valves: Vec<String>,
    vent: i32 // sus??
}

impl PartialOrd for Instance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Instance {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.vent, self.flow_rate).cmp(&(other.vent, other.flow_rate))
    }
}

impl Instance {
    fn spread(&self, rooms: &HashMap<String, Valve>, helper: bool) -> Vec<Instance> {
        // list of instances, a push is an action being made.
        let mut insts: Vec<Instance> = Vec::new();
        let v = rooms.get(&self.curr).unwrap();
        let mut ele_insts: Vec<Instance> = Vec::new();

        if (v.flow > 0) & (!self.open_valves.contains(&self.curr)) {
            let mut ov_new = self.open_valves.clone();
            let flow_add = v.flow;
            ov_new.push(self.curr.clone());
            insts.push(Instance {
                curr: self.curr.clone(),
                e_curr: self.e_curr.clone(),
                flow_rate: if helper { self.flow_rate } else { self.flow_rate + flow_add },
                open_valves: ov_new,
                vent: if helper { self.vent } else { self.vent + self.flow_rate }
            });
            if helper {
                // Just this one, only this one.
                ele_insts.append(&mut insts[0].spread_ele(&rooms, flow_add));
                insts.clear();
            }
        }
        for vnt in v.next.iter() {
            insts.push(Instance {
                curr: vnt.clone(),
                e_curr: self.e_curr.clone(),
                flow_rate: self.flow_rate,
                open_valves: self.open_valves.clone(),
                vent: if helper { self.vent } else { self.vent + self.flow_rate }
            });
        }
        if helper {
            for inst in insts.iter() {
                ele_insts.append(&mut inst.spread_ele(&rooms, 0));
            }
            return ele_insts;
        }
        return insts;
    }

    fn spread_ele(&self, rooms: &HashMap<String, Valve>, flow_add: i32) -> Vec<Instance> {
        let mut insts: Vec<Instance> = Vec::new();
        let v = rooms.get(&self.e_curr).unwrap();
        if (v.flow > 0) & (!self.open_valves.contains(&self.e_curr)) {
            let mut ov_new = self.open_valves.clone();
            ov_new.push(self.e_curr.clone());
            insts.push(Instance {
                curr: self.curr.clone(),
                e_curr: self.e_curr.clone(),
                flow_rate: self.flow_rate + flow_add + v.flow,
                open_valves: ov_new,
                vent: self.vent + self.flow_rate
            });
        }
        for vnt in v.next.iter() {
            insts.push(Instance {
                curr: self.curr.clone(),
                e_curr: vnt.clone(),
                flow_rate: self.flow_rate + flow_add,
                open_valves: self.open_valves.clone(),
                vent: self.vent + self.flow_rate
            });
        }
        return insts;
    }
}

fn main() {
    let mut valves: HashMap<String, Valve> = HashMap::new();
    parse_valves(&mut valves);
    greedy_ass_bfs(&valves, false);
    greedy_ass_bfs(&valves, true);
}

fn parse_valves(valves: &mut HashMap<String, Valve>) {
    let it: &str = include_str!("../input.txt");
    let re_valve = Regex::new(r"Valve ([A-Z]+) .*=(\d+);.*valves? (([A-Z]+,? ?)+)").unwrap();
    for l in it.lines() {
        let valve_cap = re_valve.captures(l).unwrap();
        let (v, p, n) = (&valve_cap[1], &valve_cap[2].parse::<i32>().unwrap(), &valve_cap[3].split(", ").map(|a| a.to_string()).collect::<Vec<String>>().to_vec());
        valves.insert(v.to_string().clone(), Valve { flow: p.clone(), next: (*n.clone()).to_vec() });
    }
}

fn greedy_ass_bfs(valves: &HashMap<String, Valve>, helper: bool) {
    let mut states: Vec<Instance> = vec![Instance { curr: "AA".to_string(), e_curr: "AA".to_string(), flow_rate: 0, open_valves: Vec::new(), vent: 0 }];
    let mins = if helper { 26 } else { 30 };
    for _m in 0..mins {
        states = states.into_iter().flat_map(|s| s.spread(&valves, helper).into_iter()).collect();
        // Take the best ones
        states.sort();
        states = states.into_iter().rev().take(10000).collect();
        // println!("{:?}", states[0]);
    }
    if helper {
        println!("Highest flow rate (B): {:?}", states[0].vent);
    } else {
        println!("Highest flow rate (A): {:?}", states[0].vent);
    }
}