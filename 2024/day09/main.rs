use std::collections::VecDeque;

fn main() {
    let data = include_bytes!("./input.txt");
    let layout = data
        .iter()
        .enumerate()
        .map(|(idx, n)| ((idx % 2) == 0, (idx / 2) as u64, (n - b'0') as u64))
        .filter(|&(_, _, x)| x > 0)
        .collect::<Vec<_>>();

    let mut deq = layout
        .iter()
        .cloned()
        .collect::<VecDeque<(bool, u64, u64)>>();

    let mut part1 = 0;
    let mut i = 0;
    while deq.len() > 0 {
        let (is_file, file_id, repeat) = deq.pop_front().unwrap();
        if is_file {
            part1 += file_id * (i..i + repeat).sum::<u64>();
            i += repeat;
        } else {
            while let Some((tail_is_file, tail_file_id, tail_repeat)) = deq.pop_back() {
                if tail_is_file {
                    let files_moved = if tail_repeat <= repeat {
                        deq.push_front((false, file_id, repeat - tail_repeat));
                        tail_repeat
                    } else {
                        deq.push_back((true, tail_file_id, tail_repeat - repeat));
                        repeat
                    };

                    part1 += tail_file_id * (i..i + files_moved).sum::<u64>();
                    i += files_moved;
                    break;
                }
            }
        }
    }

    println!("part1 {}", part1);

    let (_, mut holes, files) = layout
        .iter()
        .fold((0, Vec::new(), Vec::new()), |mut acc, &elem| {
            let (is_file, file_id, repeat) = elem;
            if !is_file {
                acc.1.push((acc.0, repeat))
            } else {
                acc.2.push((acc.0, file_id, repeat))
            }
            acc.0 += repeat;
            acc
        });
    
    let mut part2 = 0;
    for &(i, file_id, repeat) in files.iter().rev() {
        let h = holes
            .iter()
            .position(|&(idx, hole_size)| idx < i && hole_size >= repeat);

        if let Some(hidx) = h {
            let (start_idx, hole_size) = holes[hidx];
            if hole_size == repeat {
                holes.remove(hidx);
                part2 += file_id * (start_idx..start_idx + repeat).sum::<u64>();
            } else {
                let (start_idx, hole_size) = holes[hidx];
                holes[hidx] = (start_idx + repeat, hole_size - repeat);
                part2 += file_id * (start_idx..start_idx + repeat).sum::<u64>();
            }
        } else {
            part2 += file_id * (i..i + repeat).sum::<u64>();
        }
    }

    println!("part2 {}", part2);
}
