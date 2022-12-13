use std::{collections::HashMap, vec};

#[derive(Clone)]
struct Item {
    val: usize,
    vals: HashMap<usize, usize>,
}

impl Item {
    pub fn new(val: usize) -> Self {
        Item {
            val,
            vals: HashMap::new(),
        }
    }
}

#[derive(Clone)]
struct Test {
    div: usize,
    test_true: usize,
    test_false: usize,
}

#[derive(Clone)]
struct Monkey<'a> {
    id: usize,
    items: Vec<Item>,
    op_tok: Vec<&'a str>,
    test: Test,
    inspections: usize,
}

impl<'a> Monkey<'a> {
    fn operate(&self, worry_level: usize) -> usize {
        let a = if self.op_tok[0] == "old" { worry_level } else {self.op_tok[0].parse::<usize>().unwrap()};
        let b = if self.op_tok[2] == "old" { worry_level } else {self.op_tok[2].parse::<usize>().unwrap()};
        
        return if self.op_tok[1] == "+" { a + b } else {a * b};
    }
    fn dest(&self, worry_level: usize) -> usize {
        return if worry_level % self.test.div == 0 {
            self.test.test_true
        } else {
            self.test.test_false
        }
    }
}

fn main() {
    let data: Vec<&str> = include_str!("input.txt").trim()
        .split("\n\n")
        .collect();

    let monkeys: Vec<Monkey> = data.iter()
        .map(|chunk| {
            let lines: Vec<&str> = chunk.lines().collect();
            let starting_items: Vec<Item> = lines[1].split_once(':').unwrap().1.split(",")
                .map(|x| x.trim().parse::<usize>().unwrap())
                .map(|x| Item::new(x))
                .collect();

            Monkey {
                id: lines[0].split_whitespace().last().unwrap().trim_end_matches(":").parse::<usize>().unwrap(),
                items: starting_items, 
                op_tok: lines[2].split_once('=').unwrap().1.split_whitespace().collect(),
                test: Test {
                    div: lines[3].split_whitespace().last().unwrap().parse::<usize>().unwrap(),
                    test_true: lines[4].split_whitespace().last().unwrap().parse::<usize>().unwrap(),
                    test_false: lines[5].split_whitespace().last().unwrap().parse::<usize>().unwrap(),
                },
                inspections: 0,
            }
        })
        .collect();

    let mut monkeys1 = monkeys.clone();
    let mut new_items: Vec<Vec<Item>> = vec![vec![]; monkeys1.len()];
    for _ in 1..=20 {
        monkeys1.iter_mut().enumerate().for_each(|(midx, m)| {
            m.items.append(&mut new_items[midx]);

            m.items.iter()
                .map(|item| item.val)
                .collect::<Vec<usize>>()
                .drain(..)
                .for_each(|item| {
                    m.inspections += 1;
                    let new_worry_level = m.operate(item) / 3;
                    let target = m.dest(new_worry_level);
                    new_items[target].push(Item::new(new_worry_level));
            });

            m.items.clear();
        });
    }

    let mut part1: Vec<usize> = monkeys1.iter().map(|m| m.inspections).collect();
    part1.sort();
    println!("part1 {}", part1.iter().rev().take(2).product::<usize>());


    let mut monkeys2 = monkeys.clone();
    let divisors: Vec<usize> = monkeys.iter().map(|m| m.test.div).collect();

    monkeys2.iter_mut()
        .flat_map(|m| m.items.iter_mut())
        .for_each(|item| {
            item.vals = HashMap::from_iter(divisors.iter().map(|&x| (x, item.val % x)));
        });
    let mut new_items: Vec<Vec<Item>> = vec![vec![]; monkeys2.len()];

    for _ in 1..=10000 {
        monkeys2.iter_mut()
            .enumerate()
            .for_each(|(midx, m)| {
                m.items.append(&mut new_items[midx]);

                m.items
                    .clone()
                    .drain(..)
                    .into_iter()
                    .for_each(|mut item| {
                        m.inspections += 1;

                        item.vals.iter_mut().for_each(|(k, v)| {
                            let new_val = m.operate(*v);
                            *v = new_val % k;
                        });

                        let new_worry_level = *item.vals.get(&m.test.div).unwrap();
                        let target = m.dest(new_worry_level);
                        new_items[target].push(item);
                    });

                m.items.clear();
            });
    }

    let mut part2: Vec<usize> = monkeys2.iter().map(|m| m.inspections).collect();
    part2.sort();
    println!("part1 {}", part2.iter().rev().take(2).product::<usize>());
}

