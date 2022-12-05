fn main() {
    let (grid, moves) = include_str!("./input.txt")
        .trim_end()
        .split_once("\n\n")
        .unwrap();
    
    let mut stacks: Vec<Vec<char>> = grid.split('\n')
        .rev()
        .skip(1)
        .fold(vec![Vec::<char>::new(); 9], |mut acc, x| {
            for i in 0..9 {
                // [D] [T] [V] [M] [J] [N] [F] [M] [G]
                acc[i].push(x.chars().nth(1+i*4).unwrap());
            }
            acc
        });
    
    let moves: Vec<(usize, usize, usize)> = moves.split('\n')
        .map(|line| {
            let tokens: Vec<&str> = line.split(' ').collect();
            (tokens[1].parse().unwrap(), tokens[3].parse().unwrap(), tokens[5].parse().unwrap())
        })
        .collect();

    for (num, from, to) in moves {
        for s in stacks.clone() {
            for sp in s {
                print!("{} ", sp);
            }
            println!();
        }
        let to_pop = stacks[from].len()-num-1;
        println!("move {} elems from {} to {}", num, from, to);
        let mut removed: Vec<char> = stacks[from].drain(to_pop..).rev().collect();
        stacks[to].append(&mut removed);
    }


        
}


