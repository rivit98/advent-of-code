use std::collections::{HashMap, HashSet};

type P = (i64, i64);

struct State {
    pos: P,
    walls: HashSet<P>,
    boxes: HashSet<P>,
    boxes2: HashSet<P>,
    box_ids: HashMap<P, usize>,
}

fn do_move(state: &mut State, new_pos: P, dir: &P) -> bool {
    if state.walls.contains(&new_pos) {
        return false;
    }

    if !state.boxes.contains(&new_pos) {
        return true;
    }

    let n = (new_pos.0 + dir.0, new_pos.1 + dir.1);
    if do_move(state, n, dir) {
        state.boxes.remove(&new_pos);
        state.boxes.insert(n);
        return true;
    }

    false
}

fn do_move2(state: &mut State, pos: P, dir: &P) -> bool {
    if state.walls.contains(&pos) {
        return false;
    }

    let left_box_contains = state.boxes.contains(&pos);
    let right_box_contains2 = state.boxes2.contains(&pos);
    if !left_box_contains && !right_box_contains2 {
        return true;
    }
    
    match dir.0 {
        1 | -1 => {
            let (box_left_side, box_right_side) = if left_box_contains {
                (
                    (pos.0 + dir.0, pos.1 + dir.1),
                    (pos.0 + dir.0 + 1, pos.1 + dir.1),
                )
            } else {
                (
                    (pos.0 + dir.0 - 1, pos.1 + dir.1),
                    (pos.0 + dir.0, pos.1 + dir.1),
                )
            };
            if do_move2(state, if dir.0 == 1 { box_right_side } else {box_left_side}, dir) {
                let (cur_box_left_side, cur_box_right_side) = if left_box_contains {
                    (
                        (pos.0, pos.1),
                        (pos.0 + 1, pos.1),
                    )
                } else {
                    (
                        (pos.0 - 1, pos.1),
                        (pos.0, pos.1),
                    )
                };
                
                state.boxes.remove(&cur_box_left_side);
                state.boxes2.remove(&cur_box_right_side);
                state.boxes2.insert(box_left_side);
                state.boxes.insert(box_right_side);
                return true;
            }
        },
        _ => { // ^ v
            let (box_left_side, box_right_side) = if left_box_contains {
                (
                    (pos.0 + dir.0, pos.1 + dir.1),
                    (pos.0 + dir.0 + 1, pos.1 + dir.1),
                )
            } else {
                (
                    (pos.0 + dir.0 - 1, pos.1 + dir.1),
                    (pos.0 + dir.0, pos.1 + dir.1),
                )
            };
            if do_move2(state, box_left_side, dir) && do_move2(state, box_right_side, dir) {
                let (cur_box_left_side, cur_box_right_side) = if left_box_contains {
                    (
                        (pos.0, pos.1),
                        (pos.0 + 1, pos.1),
                    )
                } else {
                    (
                        (pos.0 - 1, pos.1),
                        (pos.0, pos.1),
                    )
                };
                
                state.boxes.remove(&cur_box_left_side);
                state.boxes2.remove(&cur_box_right_side);
                state.boxes.insert(box_left_side);
                state.boxes2.insert(box_right_side);
                return true;
            }
        },
    }
    

    false
}

fn main() {
    let (map, moves) = include_str!("input.txt").split_once("\n\n").unwrap();
    let moves = moves
        .chars()
        .map(|c| match c {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => (0, 0),
        })
        .collect::<Vec<_>>();

    let mut pos = (0, 0);
    let mut boxes = HashSet::new();
    let mut walls = HashSet::new();
    map.split('\n').enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| match c {
            '@' => pos = (x as i64, y as i64),
            '#' => {
                walls.insert((x as i64, y as i64));
                ()
            }
            'O' => {
                boxes.insert((x as i64, y as i64));
                ()
            }
            _ => (),
        })
    });

    let boxes2 = HashSet::new();
    let mut box_ids = HashMap::new();
    let mut state = State {
        pos,
        walls,
        boxes,
        boxes2,
        box_ids,
    };

    moves.iter().for_each(|&(dx, dy)| {
        let new_pos = (state.pos.0 + dx, state.pos.1 + dy);
        if do_move(&mut state, new_pos, &(dx, dy)) {
            state.pos = new_pos;
        }
    });

    let part1 = state
        .boxes
        .iter()
        .map(|&pos| pos.0 + pos.1 * 100)
        .sum::<i64>();
    println!("part 1: {}", part1);

    let mut pos = (0, 0);
    let mut boxes = HashSet::new();
    let mut boxes2 = HashSet::new();
    let mut box_ids = HashMap::new();
    let mut walls = HashSet::new();
    let mut id = 0;
    map.replace("O", "[]")
        .replace("@", "@.")
        .replace(".", "..")
        .replace("#", "##")
        .split('\n')
        .enumerate()
        .for_each(|(y, l)| {
            l.chars().enumerate().for_each(|(x, c)| {
                let p = (x as i64, y as i64);
                match c {
                    '@' => pos = (x as i64, y as i64),
                    '#' => {
                        walls.insert((x as i64, y as i64));
                        ()
                    }
                    '[' => {
                        boxes.insert((x as i64, y as i64));
                        box_ids.insert(p, id);
                        ()
                    }
                    ']' => {
                        boxes2.insert((x as i64, y as i64));
                        box_ids.insert(p, id);
                        id += 1;
                        ()
                    }
                    _ => (),
                }
            })
        });

    let mut state = State {
        pos,
        walls,
        boxes,
        boxes2,
        box_ids,
    };

    let mut m = String::new();
    for y in 0..7 {
        for x in 0..14 {
            if state.walls.contains(&(x as i64, y as i64)) {
                m.push('#');
            } else if state.boxes.contains(&(x as i64, y as i64)) {
                m.push('[');
            } else if state.boxes2.contains(&(x as i64, y as i64)) {
                m.push(']');
            } else if state.pos == (x as i64, y as i64) {
                m.push('@');
            } else {
                m.push('.')
            }
        }
        m.push('\n');
    }
    println!("{}\n", m);

    moves.iter().for_each(|&(dx, dy)| {
        let new_pos = (state.pos.0 + dx, state.pos.1 + dy);
        if do_move2(&mut state, new_pos, &(dx, dy)) {
            state.pos = new_pos;
        }

        let mut m = String::new();
        for y in 0..7 {
            for x in 0..14 {
                let p = &(x as i64, y as i64);
                if state.walls.contains(p) {
                    m.push('#');
                } else if state.boxes.contains(p) {
                    m.push('[');
                } else if state.boxes2.contains(p) {
                    m.push(']');
                } else if state.pos == (x as i64, y as i64) {
                    m.push('@');
                } else {
                    m.push('.')
                }
            }
            m.push('\n');
        }
        println!("{}\n", m);
    });

    let part2 = state
        .boxes
        .iter()
        .map(|&pos| pos.0 + pos.1 * 100)
        .sum::<i64>();
    println!("part 2: {}", part2);
}
