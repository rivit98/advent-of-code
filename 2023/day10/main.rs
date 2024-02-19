use std::collections::{HashMap, HashSet};

type P = (i64, i64);

fn explore(map: &Vec<Vec<char>>, current: P) -> (char, Vec<P>) {
    let moves = HashMap::from([
        ('|', [(1, 0), (-1, 0)]),
        ('-', [(0, 1), (0, -1)]),
        ('L', [(-1, 0), (0, 1)]),
        ('J', [(-1, 0), (0, -1)]),
        ('7', [(0, -1), (1, 0)]),
        ('F', [(0, 1), (1, 0)]),
    ]);

    let (x, y) = current;
    let (mx, my) = (map.len() as i64, map[0].len() as i64);

    // find initial pipe directions
    let candidates = vec![((0, -1), "-LF"), ((0, 1), "-J7"), ((-1, 0), "|7F"), ((1, 0), "|JL")]
        .into_iter()
        .map(|((dx, dy), str)| ((x + dx, y + dy), str))
        .filter(|&((nx, ny), _)| nx >= 0 && nx < mx && ny >= 0 && ny < my)
        .filter(|&((px, py), str)| str.contains(map[px as usize][py as usize]))
        .collect::<Vec<(P, &str)>>();

    let first = candidates.first().unwrap().0;
    let second = candidates.last().unwrap().0;
    let first_diff = (first.0-x, first.1-y);
    let second_diff = (second.0-x, second.1-y);

    let start_part = moves.iter()
        .filter(|&(_c, &[a, b])| a == first_diff || b == first_diff)
        .filter(|&(_c, &[a, b])| a == second_diff || b == second_diff)
        .next()
        .unwrap().0;

    let candidate = candidates.iter().next().unwrap();
    let mut path = vec![current];
    let mut came_from = current;
    let (mut current, _) = candidate;
    loop {
        path.push(current);

        let (x, y) = current;
        let c = map[x as usize][y as usize];
        if c == 'S' {
            break;
        }

        let current_opt = moves.get(&c).unwrap();
        let next_delta = if (x + current_opt[0].0, y + current_opt[0].1) == came_from {
            current_opt[1]
        } else {
            current_opt[0]
        };

        came_from = current;
        current = (x + next_delta.0, y + next_delta.1);
    }

    return (*start_part, path);
}

#[derive(PartialEq)]
pub enum ScanState {
    None,
    OpeningF,
    OpeningL,
}


fn main() {
    let mut map: Vec<Vec<char>> = include_str!("./input")
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let (mx, my) = (map.len() as i64, map[0].len() as i64);
    let start = (0..mx)
        .flat_map(|x| (0..my).map(move |y| (x, y)))
        .find(|&(x, y)| map[x as usize][y as usize] == 'S')
        .unwrap();

    let (s_char, path) = explore(&map, start);
    println!("part1: {}", path.len() / 2);

    map[start.0 as usize][start.1 as usize] = s_char;
    let path_set: HashSet<P> = HashSet::from_iter(path);

    let part2 = (0..mx)
        .flat_map(|x| (0..my).map(move |y| (x, y)))
        .filter(|&p| !path_set.contains(&p))
        .into_iter()
        .filter(|&p| {
            let mut crossings = 0;

            let mut state = ScanState::None;
            for y in p.1+1..my {
                let c = path_set.get(&(p.0, y)).map_or('.', |&(x,y)| map[x as usize][y as usize]);
                state = match c {
                    '|' => {
                        crossings += 1;
                        ScanState::None
                    },
                    'F' => {
                        ScanState::OpeningF
                    },
                    'L' => {
                        ScanState::OpeningL
                    },
                    '7' => {
                        if state == ScanState::OpeningL {
                            crossings += 1;
                        }
                        ScanState::None
                    },
                    'J' => {
                        if state == ScanState::OpeningF {
                            crossings += 1;
                        }
                        ScanState::None
                    },
                    _ => state
                }
            }

            return crossings % 2 != 0;
        })
        .count();

    println!("part2: {}", part2);
}