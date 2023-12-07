use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Ord, Eq, PartialOrd, PartialEq)]
enum Type {
    FIVE,
    FOUR,
    FULL,
    THREE,
    TWO_PAIR,
    PAIR,
    HIGH,
}

const CARDS: [char; 13] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
const CARDS2: [char; 13] = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];

type Hand = (&'static str, Type, u64);

fn get_type(card: &str, joker: bool) -> Type {
    let mut counts: HashMap<char, u64> = card.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_default() += 1;
        acc
    });
    let jokers = match joker {
        true => counts.remove(&'J').unwrap_or_default(),
        false => 0
    };
    let mut vals: Vec<u64> = counts.values().cloned().collect();
    if vals.is_empty() {
        vals.push(0);
    }
    vals.sort_by(|a, b| b.cmp(a));
    vals[0] += jokers;

    match vals[0] {
        5 => Type::FIVE,
        4 => Type::FOUR,
        3 => {
            match vals[1] {
                2 => Type::FULL,
                _ => Type::THREE,
            }
        }
        2 => {
            match vals[1] {
                2 => Type::TWO_PAIR,
                _ => Type::PAIR,
            }
        }
        _ => Type::HIGH
    }
}

fn main() {
    let data: Vec<_> = include_str!("./input").lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            (cards.trim(), bid.parse::<u64>().unwrap())
        })
        .collect();

    let comparator = |set: [char; 13]| {
        move |(c1, ct1, _): &Hand, (c2, ct2, _): &Hand| {
            match ct1.cmp(&ct2) {
                Ordering::Equal => {
                    for (card1, card2) in c1.chars().zip(c2.chars()) {
                        let idx1 = set.iter().position(|c| *c == card1).unwrap();
                        let idx2 = set.iter().position(|c| *c == card2).unwrap();
                        let cmp = idx1.cmp(&idx2);
                        if cmp != Ordering::Equal {
                            return cmp;
                        }
                    }
                    Ordering::Equal
                }
                other => other
            }
        }
    };

    let get_win = |set: &Vec<Hand>| -> u64 {
        set.iter()
            .rev()
            .enumerate()
            .map(|(idx, (_, _, bid))| (idx + 1) as u64 * bid)
            .sum()
    };

    let mut data1: Vec<Hand> = data.iter()
        .map(|&(cards, bid)| (cards, get_type(cards, false), bid))
        .collect();

    data1.sort_by(comparator(CARDS));
    let part1: u64 = get_win(&data1);
    println!("{part1}");

    let mut data2: Vec<_> = data.iter()
        .map(|&(cards, bid)| (cards, get_type(cards, true), bid))
        .collect();

    data2.sort_by(comparator(CARDS2));
    let part2: u64 = get_win(&data2);
    println!("{part2}");
}