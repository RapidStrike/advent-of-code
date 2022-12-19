use regex::Regex;
use std::cmp::Ordering;

// Material order goes Ore, Clay, Obsidian.
// Robot order goes Ore, Clay, Obsidian, Geode.
#[derive(Debug, Eq, PartialEq, Clone)]
struct Instance {
    bp_id: i32,
    geode: i32,
    mats: (i32, i32, i32),
    robo_count: (i32, i32, i32, i32)
}

#[derive(Debug, PartialEq, Eq)]
enum Bot {
    None,
    Ore,
    Clay,
    Obsidian,
    Geode
}

impl Instance {
    fn tick(&self, mat_cost: &Vec<MatCost>, max_ore_cost: i32) -> Vec<Instance> {
        let mut insts: Vec<Instance> = Vec::new();
        // for r in mat_cost.iter() {
        //     if self.mats.0 >= r.ore && self.mats.1 >= r.clay && self.mats.2 >= r.obsidian {
        //         insts.push(self.update(r));
        //     }
        // }
        if self.robo_count.0 < max_ore_cost && self.mats.0 >= mat_cost[3].ore {
            insts.push(self.update(&mat_cost[3]));
        }
        if self.robo_count.1 < mat_cost[1].clay && self.mats.0 >= mat_cost[2].ore {
            insts.push(self.update(&mat_cost[2]));
        }
        if self.robo_count.2 < mat_cost[0].obsidian && self.mats.0 >= mat_cost[1].ore && self.mats.1 >= mat_cost[1].clay {
            insts.push(self.update(&mat_cost[1]));
        }
        if self.mats.0 >= mat_cost[0].ore && self.mats.2 >= mat_cost[0].obsidian {
            insts.push(self.update(&mat_cost[0]));
        } else {
            insts.push(self.update(&mat_cost[4]));
        }
        insts
    }

    fn update(&self, mat_cost: &MatCost) -> Instance {
        let mut upd = self.clone();
        upd.mats = (
            self.mats.0 + self.robo_count.0 - mat_cost.ore,
            self.mats.1 + self.robo_count.1 - mat_cost.clay,
            self.mats.2 + self.robo_count.2 - mat_cost.obsidian
        );
        upd.geode = self.geode + self.robo_count.3;
        upd.robo_count = (
            if mat_cost.bot == Bot::Ore { self.robo_count.0 + 1 } else { self.robo_count.0 },
            if mat_cost.bot == Bot::Clay { self.robo_count.1 + 1 } else { self.robo_count.1 },
            if mat_cost.bot == Bot::Obsidian { self.robo_count.2 + 1 } else { self.robo_count.2 },
            if mat_cost.bot == Bot::Geode { self.robo_count.3 + 1 } else { self.robo_count.3 }
        );
        return upd;
    }
}

impl PartialOrd for Instance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Instance {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.geode, self.robo_count.3, self.robo_count.2, self.robo_count.1, self.robo_count.0).cmp(&(other.geode, other.robo_count.3, other.robo_count.2, other.robo_count.1, other.robo_count.0))
    }
}

#[derive(Debug)]
struct MatCost {
    bot: Bot,
    ore: i32,
    clay: i32,
    obsidian: i32
}

fn main() {
    let re_parse = Regex::new(r"Blueprint (\d+):.* (\d+) .* (\d+) .* (\d+) .* (\d+) .* (\d+) .* (\d+) .*").unwrap();
    bfs_but_worse(&re_parse, false);
    bfs_but_worse(&re_parse, true);
}

fn bfs_but_worse(re_parse: &Regex, part_b: bool) {
    let it: &str = include_str!("../input.txt");
    let mut q_level_a: i32 = 0;
    let mut max_size_b: i32 = 1;
    let len: usize = if part_b { 3 } else { it.lines().count() };
    let mins: usize = if part_b { 32 } else { 24 };
    let mut iter: usize = 0;
    for l in it.lines() {
        let nums = re_parse.captures(l).unwrap();
        let bot_cost: Vec<MatCost> = vec![
            MatCost { bot: Bot::Geode, ore: nums[6].parse::<i32>().unwrap(), clay: 0, obsidian: nums[7].parse::<i32>().unwrap() },
            MatCost { bot: Bot::Obsidian, ore: nums[4].parse::<i32>().unwrap(), clay: nums[5].parse::<i32>().unwrap(), obsidian: 0 },
            MatCost { bot: Bot::Clay, ore: nums[3].parse::<i32>().unwrap(), clay: 0, obsidian: 0 },
            MatCost { bot: Bot::Ore, ore: nums[2].parse::<i32>().unwrap(), clay: 0, obsidian: 0 },
            MatCost { bot: Bot::None, ore: 0, clay: 0, obsidian: 0 }
        ];
        let mut instances: Vec<Instance> = vec![Instance {
            bp_id: nums[1].parse::<i32>().unwrap(),
            geode: 0,
            mats: (0, 0, 0),
            robo_count: (1, 0, 0, 0)
        }];
        let max_ore_cost: i32 = *[nums[2].parse::<i32>().unwrap(), nums[3].parse::<i32>().unwrap(), nums[4].parse::<i32>().unwrap(), nums[6].parse::<i32>().unwrap()].iter().max().unwrap();
        for _tick in 0..mins {
            instances = instances.into_iter().flat_map(|i| i.tick(&bot_cost, max_ore_cost).into_iter()).collect();
            instances.sort();
            instances = instances.into_iter().rev().take(150000).collect();
        }
        q_level_a += instances[0].bp_id * instances[0].geode;
        max_size_b *= instances[0].geode;
        println!("ID: {} --- {:?}", &nums[1], instances[0]);
        iter += 1;
        if iter >= len {
            break;
        }
    }
    if !part_b {
        println!("Total Quality Level (A): {}", q_level_a);
    } else {
        println!("Max Geode Number (B): {}", max_size_b);
    }
}
