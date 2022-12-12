
struct Test {
    div: usize,
    test_true: usize,
    test_false: usize,
}

struct Monkey<'a> {
    items: Vec<usize>,
    op_tok: Vec<&'a str>,
    test: Test,
    inspections: usize,
}

impl<'a> Monkey<'a> {
    fn operate(&self, worry_level: usize) -> usize {
        let a = if self.op_tok[0] == "old" { worry_level } else {self.op_tok[0].parse::<usize>().unwrap()};
        let b = if self.op_tok[2] == "old" { worry_level } else {self.op_tok[2].parse::<usize>().unwrap()};
        
        return if self.op_tok[1] == "+" { a + b} else {a * b};
    }
    fn dest(&self, worry_level: usize) -> usize {
        if worry_level % self.test.div == 0 {
            return self.test.test_true;
        } else {
            return self.test.test_false;
        }
    }
}

fn main() {
    let data: Vec<&str> = include_str!("input.txt").trim()
        .split("\n\n")
        .collect();

    let mut monkeys: Vec<Monkey> = data.iter()
        .map(|chunk| {
            let lines: Vec<&str> = chunk.lines().collect();
            let starting_items: Vec<usize> = lines[1].split_once(':').unwrap().1.split(",").map(|x| x.trim().parse::<usize>().unwrap()).collect();
            let operands: Vec<&str> = lines[2].split_once('=').unwrap().1.split_whitespace().collect();

            Monkey { 
                items: starting_items, 
                op_tok: operands, 
                test: Test {
                    div: lines[3].split_whitespace().last().unwrap().parse::<usize>().unwrap(),
                    test_true: lines[4].split_whitespace().last().unwrap().parse::<usize>().unwrap(),
                    test_false: lines[5].split_whitespace().last().unwrap().parse::<usize>().unwrap(),
                },
                inspections: 0,
            }
        })
        .collect();

    for _ in 1..=20 {
        for midx in 0..monkeys.len() {
            for idx in 0..monkeys[midx].items.len() {
                monkeys[midx].inspections += 1;
                let item = monkeys[midx].items[idx];
                let new_worry_level = monkeys[midx].operate(item) / 3;
                let target = monkeys[midx].dest(new_worry_level);
                monkeys[target].items.push(new_worry_level);
            }

            monkeys[midx].items.clear();
        }
    }

    let mut part1: Vec<usize> = monkeys.iter().map(|m| m.inspections).collect();
    part1.sort();
    println!("part1 {}", part1.iter().rev().take(2).product::<usize>())

}

