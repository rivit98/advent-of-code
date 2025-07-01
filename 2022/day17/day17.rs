use std::collections::HashSet;

const BOUND_X1: i64 = 0;
const BOUND_X2: i64 = 6;

type Point = (i64, i64);

pub trait Shape {
    fn positions(&self, pos: Point) -> Vec<Point>;
}

struct MinusShape {}

struct PlusShape {}

struct LShape {}

struct BarShape {}

struct SquareShape {}

fn collide(pos: &Point, map: &HashSet<Point>) -> bool {
    let (x, y) = pos;
    if *x < BOUND_X1 || *x > BOUND_X2 || *y < 0 {
        return true;
    }

    return map.contains(pos);
}

impl Shape for MinusShape {
    // ####
    fn positions(&self, pos: Point) -> Vec<Point> {
        let (x, y) = pos;
        return (x..x + 4).into_iter().map(|nx| (nx, y)).collect();
    }
}

impl Shape for PlusShape {
    // .#.
    // ###
    // .#.
    fn positions(&self, pos: Point) -> Vec<Point> {
        let (x, y) = pos;
        return vec![(x + 1, y), (x + 1, y + 1), (x + 1, y + 2), (x, y + 1), (x + 2, y + 1)];
    }
}

impl Shape for LShape {
    // ..#
    // ..#
    // ###
    fn positions(&self, pos: Point) -> Vec<Point> {
        let (x, y) = pos;
        return vec![(x, y), (x + 1, y), (x + 2, y), (x + 2, y + 1), (x + 2, y + 2)];
    }
}

impl Shape for BarShape {
    // #
    // #
    // #
    // #
    fn positions(&self, pos: Point) -> Vec<Point> {
        let (x, y) = pos;
        return (y..y + 4).into_iter().map(|ny| (x, ny)).collect();
    }
}

impl Shape for SquareShape {
    // ##
    // ##
    fn positions(&self, pos: Point) -> Vec<Point> {
        let (x, y) = pos;
        return vec![(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)];
    }
}

fn visualize(map: &HashSet<Point>, piece: Option<&Vec<Point>>) {
    let maxy = map.iter().map(|p| p.1).max().unwrap() + 3;
    let miny = map.iter().map(|p| p.1).min().unwrap();
    let piece_set: HashSet<Point> = HashSet::from_iter(piece.unwrap_or(&Vec::new()).iter().cloned());
    let maxy = maxy.max(piece_set.iter().map(|p| p.1).max().unwrap());
    let miny = miny.min(piece_set.iter().map(|p| p.1).min().unwrap());
    for y in (miny..=maxy).rev() {
        print!("|");
        for x in BOUND_X1..=BOUND_X2 {
            if piece_set.contains(&(x,y)){
                print!("@");
            }
            else if map.contains(&(x,y)){
                print!("#");
            }else{
                print!(".");
            }
        }
        print!("|");
        println!()
    }
    println!("\n")
}

fn main() {
    let jets: Vec<char> = include_str!("input.txt").trim().chars().collect();
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(MinusShape {}),
        Box::new(PlusShape {}),
        Box::new(LShape {}),
        Box::new(BarShape {}),
        Box::new(SquareShape {}),
    ];

    let mut shapes_iter = shapes.iter().cycle();
    let mut jets_iter = jets.iter().cycle();
    let mut map: HashSet<Point> = HashSet::from_iter(
        (0..7).map(|i| (i, 0))
    );
    let mut pos = (2, 4);
    let mut current_shape = shapes_iter.next().unwrap();
    let mut rock_idx = 0;

    loop {
        let mut new_pos = match jets_iter.next().unwrap() {
            '>' => (pos.0 + 1, pos.1),
            _ => (pos.0 - 1, pos.1),
        };

        if current_shape.positions(new_pos).iter().any(|p| collide(p, &map)) {
            // no move, restore pos
            new_pos = pos;
        }

        let new_pos2 = (new_pos.0, new_pos.1 - 1);
        let positions = current_shape.positions(new_pos2);
        if positions.iter().any(|p| collide(p, &map)) {
            // cannot move anymore
            let positions = current_shape.positions(new_pos);
            visualize(&map, Some(&positions));

            map.extend(positions);
            let maxy = map.iter().map(|p| p.1).max().unwrap(); //optimize, store ony one max and update if needed
            pos = (2, maxy + 4);
            rock_idx += 1;
            if rock_idx == 2022 {
                println!("part1: {maxy}");
                break;
            }
            current_shape = shapes_iter.next().unwrap();
        } else {
            pos = new_pos2;
        }

    }
}
