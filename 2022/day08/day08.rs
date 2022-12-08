use std::cmp;

fn transpose(v: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<i32>>())
        .collect()
}

fn main() {
    let map: Vec<Vec<i32>> = include_str!("./input.txt")
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| c as i32 - 0x30).collect())
        .collect();

    let map_transposed = transpose(&map);

    let mut visible = 0;
    let mut scenic_score = 0;
    for i in 0..map.len() {
        for j in 0..map.len() {
            let num = map[i][j];

            let row_left = &map[i][..j];
            let row_right = &map[i][j+1..];
            let column_up= &map_transposed[j][..i];
            let column_down = &map_transposed[j][i+1..];
            if row_left.iter().max().map_or(-1, |x| *x) < num 
                || row_right.iter().max().map_or(-1, |x| *x) < num 
                || column_up.iter().max().map_or(-1, |x| *x) < num 
                || column_down.iter().max().map_or(-1, |x| *x) < num {
                    visible += 1;
            }

            let score: usize = vec![
                row_left, 
                row_right, 
                column_down, 
                column_up]
                    .iter()
                    .enumerate()
                    .map(|(idx, data)| {
                        let mut iter: Box<dyn Iterator<Item = _>> = if idx == 0 || idx == 3 { //reverse iter if we are checking left or upper part
                            Box::new(data.iter().rev())
                        }else{
                            Box::new(data.iter())
                        };
                        let cnt = iter
                            .try_fold(0, |acc, &tree| {
                                let ret = match num.cmp(&tree) {
                                    cmp::Ordering::Equal => Err(acc+1),
                                    cmp::Ordering::Less => Err(acc+1),
                                    cmp::Ordering::Greater => Ok(acc+1),
                                };
                                return ret;
                            });

                        let cnt = match cnt {
                            Ok(acc) => acc,
                            Err(acc) => acc
                        };

                        return cnt as usize;
                    })
                    .collect::<Vec<usize>>()
                    .iter()
                    .map(|x| {
                        return x
                    })
                    .product();
            scenic_score = cmp::max(scenic_score, score);
        }
    }

    println!("part1 {}", visible);
    println!("part2 {}", scenic_score);
}


