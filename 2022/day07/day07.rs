use core::slice::Iter;

struct Dir {
    dirs: Vec<Dir>,
    tot_size: usize,
}

fn parse_dir(iter: &mut Iter<&str>) -> Dir {
    let mut current_dir = Dir {
        dirs: Vec::<Dir>::new(),
        tot_size: 0
    };

    while let Some(line) = iter.next() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
    
        if line.starts_with("$") {
            if line.contains("$ cd ") {
                let target_path = tokens[2];

                if target_path == ".." {
                    break;
                }
                let parsed_dir = parse_dir(iter);
                current_dir.tot_size += parsed_dir.tot_size;
                current_dir.dirs.push(parsed_dir);
            }
        } else {
            let size = tokens[0];
            if !size.contains("dir"){
                current_dir.tot_size += size.parse::<usize>().unwrap();
            }
        }
    }

    return current_dir;
}

fn traverse1(dir: &Dir) -> usize{
    let sum: usize = dir.dirs.iter().map(|x| traverse1(x)).sum();
    return sum + (if dir.tot_size <= 100_000 { dir.tot_size } else {0});
}

fn traverse2(dir: &Dir) -> Vec<usize> {
    let mut sizes: Vec<usize> = dir.dirs.iter()
        .map(traverse2)
        .flatten()
        .collect();
    sizes.push(dir.tot_size);
    return sizes;
}

fn main() {
    let lines: Vec<&str> = include_str!("./input.txt").trim().lines().collect();

    let mut iterator = lines.iter();
    let dir = parse_dir(&mut iterator);

    println!("part1 {}", traverse1(&dir));

    const TOT_SIZE: usize = 70000000;
    const NEEDED_FREE: usize = 30000000;

    let free_diff = NEEDED_FREE - (TOT_SIZE - dir.tot_size);

    let sizes = traverse2(&dir);
    let min = sizes.into_iter()
        .filter(|&x| x >= free_diff)
        .map(|x| x - free_diff)
        .min()
        .unwrap();
        
    println!("part2 {}", min+free_diff);
}


