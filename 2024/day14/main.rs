use itertools::Itertools;
use regex::Regex;

fn main() {
    let re = Regex::new(r"-?\d+").unwrap();

    let data: Vec<Vec<i64>> = include_str!("./input.txt")
        .trim()
        .lines()
        .map(|line| {
            re.find_iter(line)
                .map(|m| m.as_str().parse().unwrap())
                .collect()
        })
        .collect();

    let (maxx, maxy) = (101, 103);
    // let (maxx, maxy) = (11, 7);
    let seconds = 100;

    let simulated = data
        .iter()
        .map(|robot| {
            let &[px, py, vx, vy] = robot.as_slice() else {
                unreachable!();
            };

            let mut nx = (px + vx * seconds) % maxx;
            let mut ny = (py + vy * seconds) % maxy;
            if nx < 0 {
                nx = maxx + nx
            }
            if ny < 0 {
                ny = maxy + ny
            }
            
            (nx, ny)
        })
        .collect::<Vec<(i64, i64)>>();

    let quadrants = simulated.iter().fold([[0; 2]; 2], |mut acc, &(nx, ny)| {
        let x_idx = nx < (maxx / 2);
        let y_idx = ny < (maxy / 2);

        if nx != maxx / 2 && ny != maxy / 2 {
            acc[x_idx as usize][y_idx as usize] += 1;
        }
        acc
    });
    println!("{:?}", quadrants.iter().flat_map(|q| q.iter()).product::<i64>());
    
    
    for seconds in 0..10000 {
        let mut plane = [['.'; 103]; 101];
        
        data
            .iter()
            .for_each(|robot| {
                let &[px, py, vx, vy] = robot.as_slice() else {
                    unreachable!();
                };

                let mut nx = (px + vx * seconds) % maxx;
                let mut ny = (py + vy * seconds) % maxy;
                if nx < 0 {
                    nx = maxx + nx
                }
                if ny < 0 {
                    ny = maxy + ny
                }

                plane[nx as usize][ny as usize] = '#'
            });

        let s = plane.iter().map(|row| row.iter().join("")).join("\n");
        if s.contains("#########") {
            println!("{}", seconds);
            break;
        }
    }
}

