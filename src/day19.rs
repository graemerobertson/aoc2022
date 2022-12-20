use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct Blueprint {
    id: u32,
    ore_robot_ore_cost: u32,
    clay_robot_ore_cost: u32,
    obsidian_robot_ore_cost: u32,
    obsidian_robot_clay_cost: u32,
    geode_robot_ore_cost: u32,
    geode_robot_obsidian_cost: u32,
}

#[derive(Copy, Clone, Debug)]
struct Inventory {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
}

#[derive(Copy, Clone, Debug)]
struct InventoryState {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
    starting_ore_robots: u32,
    starting_clay_robots: u32,
    starting_obsidian_robots: u32,
    starting_geode_robots: u32,
}

impl Blueprint {
    pub fn new_best_feasible(
        &self,
        current_best: u32,
        inventory: &Inventory,
        minutes: u32,
    ) -> bool {
        let mut inv_clone = inventory.clone();
        for _ in 0..minutes + 1 {
            let orig_inv_clone = inv_clone.clone();
            let mut clay_deduction = 0;
            let mut obsidian_deduction = 0;
            let mut ore_deduction = u32::MAX;
            let mut i = 0;
            loop {
                i += 1;
                if inv_clone.clay > self.obsidian_robot_clay_cost * i {
                    inv_clone.obsidian_robots += 1;
                    clay_deduction += self.obsidian_robot_clay_cost;
                } else {
                    break;
                }
            }
            i = 0;
            loop {
                let mut local_ore_deducation = 0;
                i += 1;
                if inv_clone.ore > self.ore_robot_ore_cost * i {
                    inv_clone.ore_robots += 1;
                    local_ore_deducation += self.ore_robot_ore_cost;
                } else {
                    ore_deduction = std::cmp::min(ore_deduction, local_ore_deducation);
                    break;
                }
                ore_deduction = std::cmp::min(ore_deduction, local_ore_deducation);
            }
            i = 0;
            loop {
                let mut local_ore_deducation = 0;
                i += 1;
                if inv_clone.ore > self.clay_robot_ore_cost * i {
                    inv_clone.clay_robots += 1;
                    local_ore_deducation += self.clay_robot_ore_cost;
                } else {
                    ore_deduction = std::cmp::min(ore_deduction, local_ore_deducation);
                    break;
                }
                ore_deduction = std::cmp::min(ore_deduction, local_ore_deducation);
            }
            i = 0;
            loop {
                i += 1;
                if inv_clone.obsidian > self.geode_robot_obsidian_cost * i {
                    inv_clone.geode_robots += 1;
                    obsidian_deduction += self.geode_robot_obsidian_cost;
                } else {
                    break;
                }
            }
            if inv_clone.ore >= ore_deduction {
                inv_clone.ore -= ore_deduction;
                inv_clone.ore += orig_inv_clone.ore_robots;
            } else {
                inv_clone.ore = orig_inv_clone.ore_robots;
            }
            if inv_clone.clay >= clay_deduction {
                inv_clone.clay -= clay_deduction;
                inv_clone.clay += orig_inv_clone.clay_robots;
            } else {
                inv_clone.clay = orig_inv_clone.clay_robots;
            }
            if inv_clone.obsidian >= obsidian_deduction {
                inv_clone.obsidian -= obsidian_deduction;
                inv_clone.obsidian += orig_inv_clone.obsidian_robots;
            } else {
                inv_clone.obsidian = orig_inv_clone.obsidian_robots;
            }
            inv_clone.geodes += orig_inv_clone.geode_robots;
        }
        if inv_clone.geodes > current_best {
            return true;
        }
        false
    }

    pub fn max_geodes_opened(
        &self,
        inventory: &mut Inventory,
        minutes: u32,
        max_geode_candidates: &mut HashSet<u32>,
        starting_inventory_at_minute_x: &Inventory,
        states: &mut HashMap<String, u32>,
        best_state_at_minute_x: &mut HashMap<u32, InventoryState>,
    ) {
        if !self.new_best_feasible(
            *max_geode_candidates.iter().max().unwrap_or(&(0 as u32)),
            &inventory,
            (minutes) as u32,
        ) {
            return;
        }
        let state = format!(
            "{}.{}.{}.{}.{}.{}.{}.{}.{}.{}.{}.{}",
            inventory.ore,
            inventory.clay,
            inventory.obsidian,
            inventory.geodes,
            inventory.ore_robots,
            inventory.clay_robots,
            inventory.obsidian_robots,
            inventory.geode_robots,
            starting_inventory_at_minute_x.ore_robots,
            starting_inventory_at_minute_x.clay_robots,
            starting_inventory_at_minute_x.obsidian_robots,
            starting_inventory_at_minute_x.geode_robots
        );
        if states.contains_key(&state) && states.get(&state).unwrap() >= &minutes {
            return;
        } else {
            states.insert(state, minutes);
        }
        let new_best_state = InventoryState {
            clay: inventory.clay,
            ore: inventory.ore,
            obsidian: inventory.obsidian,
            geodes: inventory.geodes,
            ore_robots: inventory.ore_robots,
            clay_robots: inventory.clay_robots,
            obsidian_robots: inventory.obsidian_robots,
            geode_robots: inventory.geode_robots,
            starting_ore_robots: starting_inventory_at_minute_x.ore_robots,
            starting_clay_robots: starting_inventory_at_minute_x.clay_robots,
            starting_obsidian_robots: starting_inventory_at_minute_x.obsidian_robots,
            starting_geode_robots: starting_inventory_at_minute_x.geode_robots,
        };
        if best_state_at_minute_x.contains_key(&minutes) {
            let prev_best_state = best_state_at_minute_x.get(&minutes).unwrap();
            if prev_best_state.ore >= inventory.ore
                && prev_best_state.clay >= inventory.clay
                && prev_best_state.obsidian >= inventory.obsidian
                && prev_best_state.geodes >= inventory.geodes
                && prev_best_state.ore_robots >= inventory.ore_robots
                && prev_best_state.clay_robots >= inventory.clay_robots
                && prev_best_state.obsidian_robots >= inventory.obsidian_robots
                && prev_best_state.geode_robots >= inventory.geode_robots
                && prev_best_state.starting_ore_robots >= starting_inventory_at_minute_x.ore_robots
                && prev_best_state.starting_clay_robots
                    >= starting_inventory_at_minute_x.clay_robots
                && prev_best_state.starting_obsidian_robots
                    >= starting_inventory_at_minute_x.obsidian_robots
                && prev_best_state.starting_geode_robots
                    >= starting_inventory_at_minute_x.geode_robots
            {
                return;
            } else if prev_best_state.ore <= inventory.ore
                && prev_best_state.clay <= inventory.clay
                && prev_best_state.obsidian <= inventory.obsidian
                && prev_best_state.geodes <= inventory.geodes
                && prev_best_state.ore_robots <= inventory.ore_robots
                && prev_best_state.clay_robots <= inventory.clay_robots
                && prev_best_state.obsidian_robots <= inventory.obsidian_robots
                && prev_best_state.geode_robots <= inventory.geode_robots
                && prev_best_state.starting_ore_robots <= starting_inventory_at_minute_x.ore_robots
                && prev_best_state.starting_clay_robots
                    <= starting_inventory_at_minute_x.clay_robots
                && prev_best_state.starting_obsidian_robots
                    <= starting_inventory_at_minute_x.obsidian_robots
                && prev_best_state.starting_geode_robots
                    <= starting_inventory_at_minute_x.geode_robots
            {
                best_state_at_minute_x.insert(minutes, new_best_state);
            }
        } else {
            best_state_at_minute_x.insert(minutes, new_best_state);
        }
        for i in 0..5 {
            let mut inventory_clone = inventory.clone();
            let mut new_world = false;
            if i == 0
                && inventory_clone.clay >= self.obsidian_robot_clay_cost
                && inventory_clone.ore >= self.obsidian_robot_ore_cost
            {
                inventory_clone.ore -= self.obsidian_robot_ore_cost;
                inventory_clone.clay -= self.obsidian_robot_clay_cost;
                inventory_clone.obsidian_robots += 1;
                new_world = true;
            } else if i == 1 && inventory_clone.ore >= self.clay_robot_ore_cost {
                inventory_clone.ore -= self.clay_robot_ore_cost;
                inventory_clone.clay_robots += 1;
                new_world = true;
            } else if i == 2 && inventory_clone.ore >= self.ore_robot_ore_cost {
                inventory_clone.ore -= self.ore_robot_ore_cost;
                inventory_clone.ore_robots += 1;
                new_world = true;
            } else if i == 3
                && inventory_clone.ore >= self.geode_robot_ore_cost
                && inventory_clone.obsidian >= self.geode_robot_obsidian_cost
            {
                inventory_clone.obsidian -= self.geode_robot_obsidian_cost;
                inventory_clone.ore -= self.geode_robot_ore_cost;
                inventory_clone.geode_robots += 1;
                new_world = true;
            } else if i == 4 {
                new_world = true;
            }
            inventory_clone.geodes += starting_inventory_at_minute_x.geode_robots;
            inventory_clone.obsidian += starting_inventory_at_minute_x.obsidian_robots;
            inventory_clone.clay += starting_inventory_at_minute_x.clay_robots;
            inventory_clone.ore += starting_inventory_at_minute_x.ore_robots;
            let new_gdr = inventory_clone.clone();

            if minutes > 1 {
                if new_world {
                    self.max_geodes_opened(
                        &mut inventory_clone,
                        minutes - 1,
                        max_geode_candidates,
                        &new_gdr,
                        states,
                        best_state_at_minute_x,
                    );
                }
            } else {
                max_geode_candidates.insert(inventory_clone.geodes);
            }
        }
    }
}

pub(crate) fn day19() {
    let f: File = File::open("data/day19.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let input_data: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let mut blueprints: Vec<Blueprint> = vec![];
    for line in input_data {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$").unwrap();
        }
        let cap = RE.captures(&line).unwrap();
        blueprints.push(Blueprint {
            id: cap[1].parse::<u32>().unwrap(),
            ore_robot_ore_cost: cap[2].parse::<u32>().unwrap(),
            clay_robot_ore_cost: cap[3].parse::<u32>().unwrap(),
            obsidian_robot_ore_cost: cap[4].parse::<u32>().unwrap(),
            obsidian_robot_clay_cost: cap[5].parse::<u32>().unwrap(),
            geode_robot_ore_cost: cap[6].parse::<u32>().unwrap(),
            geode_robot_obsidian_cost: cap[7].parse::<u32>().unwrap(),
        });
    }
    println!(
        "{:?}",
        blueprints
            .iter()
            .map(|b| {
                let mut s = HashSet::new();
                let mut m1 = HashMap::new();
                let mut m2 = HashMap::new();
                println!("Analaysing {}", b.id);
                b.max_geodes_opened(
                    &mut Inventory {
                        ore: 0,
                        clay: 0,
                        obsidian: 0,
                        geodes: 0,
                        ore_robots: 1,
                        clay_robots: 0,
                        obsidian_robots: 0,
                        geode_robots: 0,
                    },
                    32,
                    &mut s,
                    &Inventory {
                        ore: 0,
                        clay: 0,
                        obsidian: 0,
                        geodes: 0,
                        ore_robots: 1,
                        clay_robots: 0,
                        obsidian_robots: 0,
                        geode_robots: 0,
                    },
                    &mut m1,
                    &mut m2,
                );
                s.iter().max().unwrap_or(&0).clone()
            })
            .collect::<Vec<u32>>()
    );
}
