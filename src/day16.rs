use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

// This is not a safe assumption, but I need to restrict the search space somehow, and I've run
// out of other ideas.
const MAX_TIME_WITHOUT_OPENING_VALVE: usize = 9;

struct Room {
    flow_rate: u32,
    tunnels: Vec<String>,
}

#[derive(PartialEq)]
struct RoomHistory {
    current_room: String,
    previous_rooms: HashSet<String>,
}

fn calculate_max_flow(
    minutes_left: u32,
    current_rooms: Vec<RoomHistory>,
    all_rooms: &HashMap<String, Room>,
    on_pipes: Vec<String>,
    states: &mut HashMap<String, u32>,
) -> u32 {
    let mut max_flow: u32 = 0;
    let current_room1_name = &current_rooms.first().unwrap().current_room;
    let previous_room1s = &current_rooms.first().unwrap().previous_rooms;
    let current_room1 = all_rooms.get(current_room1_name).unwrap();
    let current_room2_name = &current_rooms.last().unwrap().current_room;
    let previous_room2s = &current_rooms.last().unwrap().previous_rooms;
    let current_room2 = all_rooms.get(current_room2_name).unwrap();
    let on_pipes_set: HashSet<&String> = HashSet::from_iter(on_pipes.iter());
    let current_state: String = format!(
        "{}{}{}{}",
        minutes_left,
        current_room1_name,
        current_room2_name,
        on_pipes.join("")
    );
    if states.contains_key(&current_state) {
        *states.get(&current_state).unwrap()
    } else if minutes_left > 0 {
        // Both turn on
        if (!on_pipes_set.contains(&current_room1_name) && current_room1.flow_rate > 0)
            && (!on_pipes_set.contains(&current_room2_name) && current_room2.flow_rate > 0)
            && current_room1_name != current_room2_name
        {
            let mut on_pipes_clone = on_pipes.to_owned();
            on_pipes_clone.push(current_room1_name.to_owned());
            on_pipes_clone.push(current_room2_name.to_owned());
            on_pipes_clone.sort();
            max_flow += minutes_left * current_room1.flow_rate;
            max_flow += minutes_left * current_room2.flow_rate;
            let next_rooms = vec![
                RoomHistory {
                    current_room: current_room1_name.to_owned(),
                    previous_rooms: HashSet::new(),
                },
                RoomHistory {
                    current_room: current_room2_name.to_owned(),
                    previous_rooms: HashSet::new(),
                },
            ];
            max_flow += calculate_max_flow(
                minutes_left - 1,
                next_rooms,
                all_rooms,
                on_pipes_clone,
                states,
            );
        }
        // Turn on room 1
        if previous_room2s.len() < MAX_TIME_WITHOUT_OPENING_VALVE
            && !on_pipes_set.contains(&current_room1_name)
            && current_room1.flow_rate > 0
        {
            for next_room2_name in &current_room2.tunnels {
                if !previous_room2s.contains(next_room2_name) {
                    let mut on_pipes_clone = on_pipes.to_owned();
                    on_pipes_clone.push(current_room1_name.to_owned());
                    on_pipes_clone.sort();
                    let mut potential_new_max_flow = minutes_left * current_room1.flow_rate;
                    let mut previous_room2s_clone = previous_room2s.clone();
                    previous_room2s_clone.insert(current_room2_name.to_owned());
                    let mut next_rooms = vec![
                        RoomHistory {
                            current_room: current_room1_name.to_owned(),
                            previous_rooms: HashSet::new(),
                        },
                        RoomHistory {
                            current_room: next_room2_name.to_owned(),
                            previous_rooms: previous_room2s_clone,
                        },
                    ];
                    next_rooms.sort_by(|a, b| a.current_room.cmp(&b.current_room));
                    potential_new_max_flow += calculate_max_flow(
                        minutes_left - 1,
                        next_rooms,
                        all_rooms,
                        on_pipes_clone,
                        states,
                    );
                    if potential_new_max_flow > max_flow {
                        max_flow = potential_new_max_flow;
                    }
                }
            }
        }
        // Turn on room 2
        if previous_room1s.len() < MAX_TIME_WITHOUT_OPENING_VALVE
            && !on_pipes_set.contains(&current_room2_name)
            && current_room2.flow_rate > 0
        {
            for next_room1_name in &current_room1.tunnels {
                if !previous_room1s.contains(next_room1_name) {
                    let mut on_pipes_clone = on_pipes.to_owned();
                    on_pipes_clone.push(current_room2_name.to_owned());
                    on_pipes_clone.sort();
                    let mut potential_new_max_flow = minutes_left * current_room2.flow_rate;
                    let mut previous_room1s_clone = previous_room1s.clone();
                    previous_room1s_clone.insert(current_room1_name.to_owned());
                    let mut next_rooms = vec![
                        RoomHistory {
                            current_room: current_room2_name.to_owned(),
                            previous_rooms: HashSet::new(),
                        },
                        RoomHistory {
                            current_room: next_room1_name.to_owned(),
                            previous_rooms: previous_room1s_clone,
                        },
                    ];
                    next_rooms.sort_by(|a, b| a.current_room.cmp(&b.current_room));
                    potential_new_max_flow += calculate_max_flow(
                        minutes_left - 1,
                        next_rooms,
                        all_rooms,
                        on_pipes_clone,
                        states,
                    );
                    if potential_new_max_flow > max_flow {
                        max_flow = potential_new_max_flow;
                    }
                }
            }
        }
        // Turn on neither
        if previous_room1s.len() < MAX_TIME_WITHOUT_OPENING_VALVE
            && previous_room2s.len() < MAX_TIME_WITHOUT_OPENING_VALVE
        {
            for next_room1_name in &current_room1.tunnels {
                if !previous_room1s.contains(next_room1_name) {
                    for next_room2_name in &current_room2.tunnels {
                        if !previous_room2s.contains(next_room2_name) {
                            let mut previous_room1s_clone = previous_room1s.clone();
                            previous_room1s_clone.insert(current_room1_name.to_owned());
                            let mut previous_room2s_clone = previous_room2s.clone();
                            previous_room2s_clone.insert(current_room2_name.to_owned());
                            let mut next_rooms = vec![
                                RoomHistory {
                                    current_room: next_room1_name.to_owned(),
                                    previous_rooms: previous_room1s_clone,
                                },
                                RoomHistory {
                                    current_room: next_room2_name.to_owned(),
                                    previous_rooms: previous_room2s_clone,
                                },
                            ];
                            next_rooms.sort_by(|a, b| a.current_room.cmp(&b.current_room));
                            let potential_new_max_flow = calculate_max_flow(
                                minutes_left - 1,
                                next_rooms,
                                all_rooms,
                                on_pipes.to_owned(),
                                states,
                            );
                            if potential_new_max_flow > max_flow {
                                max_flow = potential_new_max_flow;
                            }
                        }
                    }
                }
            }
        }
        states.insert(current_state, max_flow);
        max_flow
    } else {
        0
    }
}

pub(crate) fn day16() {
    let f: File = File::open("data/day16.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let input_data: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let mut rooms: HashMap<String, Room> = HashMap::new();
    for line in input_data {
        lazy_static! {
            static ref RE_MAIN: Regex = Regex::new(
                r"^Valve ([A-Z]+) has flow rate=(\d+); tunnels* leads* to valves* ([A-Z]+)"
            )
            .unwrap();
        }
        lazy_static! {
            static ref RE_TUNNELS: Regex = Regex::new(r", ([A-Z]+)").unwrap();
        }
        let cap = RE_MAIN.captures(&line).unwrap();
        let mut tunnels: Vec<String> = RE_TUNNELS
            .find_iter(&line)
            // try to parse the string matches as i64 (inferred from fn type signature)
            // and filter out the matches that can't be parsed (e.g. if there are too many digits to store in an i64).
            .filter_map(|tunnel| tunnel.as_str().strip_prefix(", ").unwrap().parse().ok())
            // collect the results in to a Vec<i64> (inferred from fn type signature)
            .collect();
        tunnels.push(cap[3].parse().unwrap());
        let room = Room {
            flow_rate: cap[2].parse::<u32>().unwrap(),
            tunnels,
        };
        rooms.insert(cap[1].parse().unwrap(), room);
    }
    let mut states: HashMap<String, u32> = HashMap::new();
    println!(
        "Maximum amount of pressure we can release is {}",
        calculate_max_flow(
            25,
            vec![
                RoomHistory {
                    current_room: "AA".to_string(),
                    previous_rooms: HashSet::new()
                },
                RoomHistory {
                    current_room: "AA".to_string(),
                    previous_rooms: HashSet::new()
                }
            ],
            &rooms,
            vec![],
            &mut states
        )
    );
}
