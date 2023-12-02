use std::cmp::max;

struct Pick {
    r: u32,
    g: u32,
    b: u32,
}

struct Game {
    id: u32,
    picks: Vec<Pick>,
}


fn main() {
    let data: Vec<Game> = include_str!("input.txt")
        .lines()
        .map(|l| {
            // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            let (game, picks) = l.split_once(':').unwrap();
            let game_id: u32 = game.split_once(' ').unwrap().1.parse().unwrap();

            return Game {
                id: game_id,
                picks: picks.split(';')
                    .map(|pick|
                        pick.split(',')
                            .map(|pick| pick.trim().split_once(' ').unwrap())
                            .fold(Pick { r: 0, g: 0, b: 0 }, |mut acc, (val, color)| {
                                let v: u32 = val.parse().unwrap();
                                match color {
                                    "red" => acc.r += v,
                                    "green" => acc.g += v,
                                    "blue" => acc.b += v,
                                    _ => panic!()
                                }
                                acc
                            }))
                    .collect(),
            };
        })
        .collect();

    let start = Pick {
        r: 12,
        g: 13,
        b: 14,
    };

    let part1: u32 = data.iter()
        .filter(|game| {
            game.picks.iter().all(|pick| pick.r <= start.r && pick.b <= start.b && pick.g <= start.g)
        })
        .map(|g| g.id)
        .sum();

    println!("{part1}");

    let part2: u32 = data.iter()
        .map(|game| {
            game.picks.iter()
                .fold(Pick { r: 0, g: 0, b: 0 }, |mut acc, p| {
                    acc.r = max(acc.r, p.r);
                    acc.g = max(acc.g, p.g);
                    acc.b = max(acc.b, p.b);
                    acc
                })
        })
        .map(|p| p.b * p.r * p.g)
        .sum();

    println!("{part2}");
}
