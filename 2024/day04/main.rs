fn main() {
    let data = include_bytes!("./input.txt");
    let maxx = data.iter().take_while(|&b| *b != b'\n').count() + 1;

    let moves = [(0, -1), (0, 1), (1, 0), (-1, 0), (1, 1), (-1, -1), (1, -1), (-1, 1)];

    let part1: usize = data
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == b'X')
        .map(|(idx, _)| {
            let x = (idx / maxx) as i64;
            let y = (idx % maxx) as i64;
            moves.iter().filter(|&(dx, dy)| {
                b"XMAS".iter().enumerate().all(|(i, &ec)| {
                    let i = i as i64;
                    let nx = x + dx * i;
                    let ny = y + dy * i;
                    let rowlen = maxx as i64;
                    let pos = nx * rowlen + ny;
                    (0..data.len() as i64).contains(&pos) && data[pos as usize] == ec
                })
            })
                .count()
        })
        .sum();

    println!("{part1}");

    let mut moves = [(-1, -1), (1, -1), (1, 1), (-1, 1)];
    let setup = "MSSM";
    let mut rotations: Vec<Vec<(char, (i64, i64))>> = vec![];
    for _ in 0..4 {
        rotations.push(setup.chars().zip(moves).collect());
        moves.rotate_right(1);
    }
    
    let part2: u64 = data
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == b'A')
        .map(|(idx, _)| {
            let x = (idx / maxx) as i64;
            let y = (idx % maxx) as i64;
            rotations.iter().any(|rotation| {
                rotation.iter().all(|&(ec, (dx,dy))| {
                    let nx = x + dx;
                    let ny = y + dy;
                    let rowlen = maxx as i64;
                    let pos = nx * rowlen + ny;
                    (0..data.len() as i64).contains(&pos) && data[pos as usize] == ec as u8
                })
            }) as u64
        })
        .sum();

    println!("{part2}");
}