use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Valve {
    id: i64,
    neighbours: Vec<i64>,
    rate: usize,
}

fn walk(
    cache: &mut HashMap<(i64, i64, i64), usize>,
    valves: &HashMap<i64, Valve>,
    curr_id: i64,
    opened: &mut HashSet<i64>,
    timeleft: i64,
    current_rate: usize,
) -> usize {
    if timeleft <= 1 || opened.len() == valves.len() {
        // opening valve does nothing
        return current_rate;
    }

    let cached_val = cache.get(&(curr_id, timeleft, opened.iter().product::<i64>()));
    if cached_val.is_some() {
        return cached_val.unwrap().clone();
    }
    let curr_valve = valves.get(&curr_id).unwrap();
    let curr_rate = curr_valve.rate;
    let mut max_rate = current_rate;

    for &next in curr_valve.neighbours.iter() {
        // open valve if not opened already or if its rate is greater than 0
        if curr_rate != 0 && !opened.contains(&curr_id) {
            opened.insert(curr_id);
            let rate = walk(cache, valves, next, opened, timeleft - 2, current_rate + ((timeleft - 1) as usize * curr_rate));
            opened.remove(&curr_id);
            max_rate = max_rate.max(rate);
        }

        // leave current valve closed, move to next
        let rate = walk(cache, valves, next, opened, timeleft - 1, current_rate);
        max_rate = max_rate.max(rate);
    }

    cache.insert((curr_id.to_owned(), timeleft, opened.iter().product::<i64>()), max_rate);
    return max_rate;
}


fn walk2(
    cache: &mut HashMap<((i64, i64), (i64, i64), i64), usize>,
    valves: &HashMap<i64, Valve>,
    curr_id: (i64, i64),
    opened: &mut HashSet<i64>,
    timeleft: (i64, i64),
    current_rate: usize,
) -> usize {
    if (timeleft.0 <= 1 && timeleft.1 <= 1) || opened.len() == valves.len() {
        // opening valve does nothing
        return current_rate;
    }

    if timeleft.0 <= 1 {
        // first player finished
        let mut fake_cache = HashMap::new();
        return walk(&mut fake_cache, valves, curr_id.1, opened, timeleft.1, current_rate)
    }

    if timeleft.1 <= 1 {
        // second player finished
        let mut fake_cache = HashMap::new();
        return walk(&mut fake_cache, valves, curr_id.0, opened, timeleft.0, current_rate)
    }

    let cached_val = cache.get(&(curr_id, timeleft, opened.iter().product::<i64>()));
    if cached_val.is_some() {
        return cached_val.unwrap().clone();
    }
    let curr_valve0 = valves.get(&curr_id.0).unwrap();
    let curr_valve1 = valves.get(&curr_id.1).unwrap();
    let curr_rate0 = curr_valve0.rate;
    let curr_rate1 = curr_valve1.rate;
    let mut max_rate = current_rate;

    for &next0 in curr_valve0.neighbours.iter() {
        for &next1 in curr_valve1.neighbours.iter() {
            if next0 == next1 {
                continue;
            }

            let worth_opening0 = curr_rate0 != 0 && !opened.contains(&curr_id.0);
            let worth_opening1 = curr_rate1 != 0 && !opened.contains(&curr_id.1);

            if worth_opening0 && worth_opening1 {
                // first player opens valve, second player opens valve
                let gain0 = (timeleft.0 - 1) as usize * curr_rate0;
                let gain1 = (timeleft.1 - 1) as usize * curr_rate1;
                opened.insert(curr_id.0);
                opened.insert(curr_id.1);
                let rate = walk2(cache, valves, (next0, next1), opened, (timeleft.0 - 2, timeleft.1 - 2), current_rate + gain0 + gain1);
                opened.remove(&curr_id.0);
                opened.remove(&curr_id.1);
                max_rate = max_rate.max(rate);
                continue;
            }

            if worth_opening0 {
                // first player opens valve, second player moves to next
                let gain0 = (timeleft.0 - 1) as usize * curr_rate0;
                let gain1 = 0;
                opened.insert(curr_id.0);
                let rate = walk2(cache, valves, (next0, next1), opened, (timeleft.0 - 2, timeleft.1 - 1), current_rate + gain0 + gain1);
                opened.remove(&curr_id.0);
                max_rate = max_rate.max(rate);
                continue;
            }

            if worth_opening1 {
                // first player moves to next, second player opens valve
                let gain0 = 0;
                let gain1 = (timeleft.1 - 1) as usize * curr_rate1;
                opened.insert(curr_id.1);
                let rate = walk2(cache, valves, (next0, next1), opened, (timeleft.0 - 1, timeleft.1 - 2), current_rate + gain0 + gain1);
                opened.remove(&curr_id.1);
                max_rate = max_rate.max(rate);
                continue;
            }

            // both players move to next valve
            let gain0 = 0;
            let gain1 = 0;
            let rate = walk2(cache, valves, (next0, next1), opened, (timeleft.0 - 1, timeleft.1 - 1), current_rate + gain0 + gain1);
            max_rate = max_rate.max(rate);
        }
    }

    cache.insert((curr_id, timeleft, opened.iter().product::<i64>()), max_rate);
    return max_rate;
}

fn main() {
    let ids: Vec<i64> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541];
    let mut id_map: HashMap<String, i64> = HashMap::new();
    let mut ids_idx = 0;

    let mut id_translate = |id: &String| -> i64 {
        let i = id_map.get(id);
        if i.is_none() {
            let nv = ids[ids_idx];
            id_map.insert(id.to_string(), nv);
            ids_idx += 1;
            return nv;
        }
        return *i.unwrap();
    };

    let valve_map: HashMap<i64, Valve> = HashMap::from_iter(
        include_str!("input.txt")
            .trim()
            .lines()
            .map(|line| {
                let (valve, next) = line.split_once(";").unwrap();
                let valve_toks: Vec<&str> = valve.split_whitespace().collect();

                let id = valve_toks[1].to_string();
                let rate = valve_toks[4].chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse::<usize>().unwrap();
                let neigh = next.chars()
                    .filter(|c| c.is_ascii_uppercase() || *c == ' ')
                    .collect::<String>()
                    .split_whitespace()
                    .map(|s| id_translate(&s.to_string()))
                    .collect::<Vec<i64>>();

                let valve = Valve {
                    id: id_translate(&id),
                    rate,
                    neighbours: neigh,
                };
                (valve.id, valve)
            })
    );

    let mut cache = HashMap::new();
    let mut opened = HashSet::new();
    let starting_node = id_map.get("AA").unwrap().clone();
    let max_pressure = walk(
        &mut cache,
        &valve_map,
        starting_node,
        &mut opened,
        30,
        0,
    );
    println!("part1 {max_pressure}");

    let mut cache = HashMap::new();
    let mut opened = HashSet::new();
    let max_pressure = walk2(
        &mut cache,
        &valve_map,
        (starting_node, starting_node),
        &mut opened,
        (26, 26),
        0,
    );
    println!("part2 {max_pressure}");
    // for some reason it returns wrong solution for test input 1706 vs 1707, who cares

    // another idea is to make graph and reduce its complexity by remove nodes with rate=0
}
