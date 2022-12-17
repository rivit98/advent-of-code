use core::slice::Iter;
use std::cmp::Ordering;

#[derive(Eq, PartialEq, Clone)]
struct Item {
    v: Option<u32>,
    l: Vec<Item>
}

impl Item {
    pub fn new() -> Item{
        Item {
            v: None,
            l: Vec::new(),
        }
    }

    pub fn from_val(v: u32) -> Item {
        Item {
            v: Some(v),
            l: Vec::new(),
        }
    }
}

fn _parse_pair(iter: &mut Iter<char>) -> Item {
    iter.next();
    iter.next_back();
    return parse_pair(iter);
}

fn parse_pair(iter: &mut Iter<char>) -> Item{
    let mut curr = Item::new();
    let mut chars: Vec<char> = Vec::new();
    while let Some(&c) = iter.next() {
        match c {
            '[' => {
                curr.l.push(parse_pair(iter));
            },
            ']' => {
                if chars.len() > 0 {
                    curr.l.push(Item::from_val(chars.iter().collect::<String>().parse::<u32>().unwrap()));
                    chars.clear();
                }
                return curr;
            },
            ',' => {
                if chars.len() > 0 {
                    curr.l.push(Item::from_val(chars.iter().collect::<String>().parse::<u32>().unwrap()));
                    chars.clear();
                }
                continue
            },
            _ => {
                chars.push(c);
            }
        }
    }
    if chars.len() > 0 {
        curr.l.push(Item::from_val(chars.iter().collect::<String>().parse::<u32>().unwrap()));
        chars.clear();
    }
    curr
}

fn validate(left: &Item, right: &Item) -> Ordering {
    let left_is_number = left.v.is_some();
    let right_is_number = right.v.is_some();
    if left_is_number && right_is_number {
        return left.v.unwrap().cmp(&right.v.unwrap());
    }

    let fake_vec = match (left_is_number, right_is_number) {
        (false, true) => Vec::from_iter([Item::from_val(right.v.unwrap())]),
        (true, false) => Vec::from_iter([Item::from_val(left.v.unwrap())]),
        _ => Vec::new(),
    };

    let mut left_iter = left.l.iter().peekable();
    let mut right_iter = right.l.iter().peekable();

    if left_is_number {
        left_iter = fake_vec.iter().peekable();
    }

    if right_is_number {
        right_iter = fake_vec.iter().peekable();
    }

    while left_iter.peek().is_some() && right_iter.peek().is_some(){
        let inner1 = left_iter.next().unwrap();
        let inner2 = right_iter.next().unwrap();

        match validate(inner1, inner2) {
            Ordering::Greater => {
                return Ordering::Greater;
            },
            Ordering::Equal => {
                continue;
            },
            Ordering::Less => {
                return Ordering::Less;
            }
        }
    }

    return left_iter.count().cmp(&right_iter.count());
}

fn main() {
    let data: Vec<&str> = include_str!("input.txt").trim().split("\n\n").collect();

    let signals: Vec<(Item, Item)> = data.iter().map(|chunk| {
        let (p1, p2) = chunk.split_once('\n').unwrap();
        let item1 = _parse_pair(&mut p1.chars().collect::<Vec<char>>().iter());
        let item2 = _parse_pair(&mut p2.chars().collect::<Vec<char>>().iter());
        (item1, item2)
    }).collect();

    let part1: usize = signals.iter()
        .map(|(s1, s2)| validate(s1, s2))
        .enumerate()
        .filter(|&(_, x)| x == Ordering::Less)
        .map(|(idx, _)| idx + 1)
        .sum();

    println!("part1 {}", part1);

    let div1 = _parse_pair(&mut "[[2]]".chars().collect::<Vec<char>>().iter());
    let div2 = _parse_pair(&mut "[[6]]".chars().collect::<Vec<char>>().iter());

    let mut all_signals: Vec<Item> = include_str!("input.txt").trim()
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|p1| _parse_pair(&mut p1.chars().collect::<Vec<char>>().iter()))
        .chain(vec![div1.clone(), div2.clone()])
        .collect();

    all_signals.sort_by(|item1, item2| validate(item1, item2));

    let p1 = all_signals.iter().position(|i| i.eq(&div1)).unwrap() + 1;
    let p2 = all_signals.iter().position(|i| i.eq(&div2)).unwrap() + 1;

    println!("part2 {}", p1 * p2);
}
