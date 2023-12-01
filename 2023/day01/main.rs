use std::collections::HashMap;

fn main() {
    let data: Vec<&str> = include_str!("./input")
        .trim()
        .lines()
        .collect();

    let part1: u32 = data
        .iter()
        .map(|line| {
            let ldigit = line.chars().find(|&v| char::is_digit(v, 10)).unwrap();
            let rdigit = line.chars().rfind(|&v| char::is_digit(v, 10)).unwrap();
            (ldigit.to_digit(10).unwrap(), rdigit.to_digit(10).unwrap())
        })
        .map(|(v1, v2)| 10 * v1 + v2).sum();
    println!("part1 {part1}");

    let mapping: HashMap<&str, u32> = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
        .iter()
        .enumerate()
        .map(|(idx, &word)| (word, (idx + 1 ) as u32))
        .collect();

    let part2: u32 = data
        .iter()
        .map(|line| {
            let ldigit = line
                .find(|c| char::is_digit(c, 10))
                .and_then(|index| Some((index, line.chars().nth(index).unwrap().to_digit(10).unwrap())));

            let rdigit = line
                .rfind(|c| char::is_digit(c, 10))
                .and_then(|index| Some((index, line.chars().nth(index).unwrap().to_digit(10).unwrap())));

            let left_word = mapping
                .keys()
                .filter_map(|&word| {
                    line.find(word).map(|index| (word, index))
                })
                .min_by_key(|&(_, index)| index)
                .and_then(|(word, index)| Some((index, *mapping.get(word).unwrap())));

            let right_word = mapping
                .keys()
                .filter_map(|&word| {
                    line.rfind(word).map(|index| (word, index))
                })
                .max_by_key(|&(_, index)| index)
                .and_then(|(word, index)| Some((index, *mapping.get(word).unwrap())));

            let l = match left_word {
                None => ldigit.unwrap().1,
                Some((word_idx, word_value)) => {
                    match ldigit {
                        None => word_value,
                        Some((index, value)) => if word_idx < index {word_value} else {value}
                    }
                }
            };

            let r = match right_word {
                None => rdigit.unwrap().1,
                Some((word_idx, word_value)) => {
                    match rdigit {
                        None => word_value,
                        Some((index, value)) => if word_idx > index {word_value} else {value}
                    }
                }
            };

            (l, r)
        })
        .map(|(v1, v2)| 10 * v1 + v2).sum();
    println!("part2 {part2:?}");
}