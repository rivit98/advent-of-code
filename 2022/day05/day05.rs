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
                let c = x.chars().nth(1+i*4).unwrap();
                if c != ' '{
                    acc[i].push(c);
                }
            }
            acc
        });
    
    stacks.insert(0, Vec::new());
    let mut part1_stacks = stacks.clone();
    let mut part2_stacks = stacks.clone();

    moves.split('\n')
        .map(|line| {
            let tokens: Vec<&str> = line.split(' ').collect();
            (tokens[1].parse::<usize>().unwrap(), tokens[3].parse::<usize>().unwrap(), tokens[5].parse::<usize>().unwrap())
        })
        .for_each(|(num, from, to)| {
            let to_pop: usize = part1_stacks[from].len()-num;
            let mut removed_part1: Vec<char> = part1_stacks[from].drain(to_pop..).rev().collect();
            let mut removed_part2: Vec<char> = part2_stacks[from].drain(to_pop..).collect();
            part1_stacks[to].append(&mut removed_part1);
            part2_stacks[to].append(&mut removed_part2);
        });
    

    let part1: String = part1_stacks.into_iter()
        .skip(1)
        .map(|x| x.last().unwrap().clone())
        .collect();

    println!("part1 {}", part1);        


    let part2: String = part2_stacks.into_iter()
        .skip(1)
        .map(|x| x.last().unwrap().clone())
        .collect();

    println!("part2 {}", part2);   
}


