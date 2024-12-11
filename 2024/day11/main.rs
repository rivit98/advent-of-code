use std::collections::HashMap;

fn expand_stone(cache: &mut HashMap<(u64, u64), u64>, stone: u64, blinks: u64) -> u64 {
    if blinks == 0 {
        return 1;
    }
    
    if cache.contains_key(&(stone, blinks)) {
        return cache[&(stone, blinks)];
    }
    
    if stone == 0 {
        let c = expand_stone(cache, 1, blinks -1);
        cache.insert((1, blinks-1), c);
        return c
    }

    let digits_num = stone.ilog10() as u64 + 1;
    if digits_num % 2 == 0 {
        let mask = 10_u64.pow((digits_num / 2) as u32);
        let c1 = expand_stone(cache, stone / mask, blinks -1);
        cache.insert((stone / mask, blinks-1), c1);
        let c2 = expand_stone(cache, stone % mask, blinks -1);
        cache.insert((stone % mask, blinks-1), c2);
        return c1 + c2
    }

    let c = expand_stone(cache, stone * 2024, blinks -1);
    cache.insert((stone * 2024, blinks-1), c);
    return c
}

fn main() {
    let stones: Vec<u64> = "337 42493 1891760 351136 2 6932 73 0".split(" ").map(|x| x.parse::<u64>().unwrap()).collect();
    
    let mut cache = HashMap::new();
    let part1 = stones.iter().map(|stone| expand_stone(&mut cache, *stone, 25)).sum::<u64>();
    println!("part1: {}", part1);

    let part2 = stones.iter().map(|stone| expand_stone(&mut cache, *stone, 75)).sum::<u64>();
    println!("part2: {}", part2);
}