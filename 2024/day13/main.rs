use itertools::Itertools;
use regex::Regex;

fn gcd(mut a:usize, mut b:usize) -> usize{
    if a==b { return a; }
    if b > a {
        let temp = a;
        a = b;
        b = temp;
    }
    while b>0 {
        let temp = a;
        a = b;
        b = temp%b;
    }
    return a;
}

fn lcm(a:usize, b:usize) -> usize{
    // LCM = a*b / gcd
    return a*(b/gcd(a,b));
}

fn main() {
    let re = Regex::new(r"\d+").unwrap();
    let data: Vec<Vec<(i64, i64)>> = re
        .find_iter(include_str!("./input.txt").trim())
        .chunks(6)
        .into_iter()
        .map(|d| {
            d.chunks(2)
                .into_iter()
                .map(|mut c| {
                    let a = c.next().unwrap().as_str().parse().unwrap();
                    let b = c.next().unwrap().as_str().parse().unwrap();
                    (a, b)
                })
                .collect()
        })
        .collect();

    // a = (s2*x2-s1*y2)/(x2*y1-x1*y2)
    // b = (s1-a*x1)/x2

    let mut part1 = 0;
    for d in data.iter() {
        let ((x1,y1),(x2,y2),(s1,s2)) = (d[0],d[1],d[2]);
        let a_nominator = s2*x2-s1*y2;
        let a_denominator = x2*y1-x1*y2;
        let a = a_nominator / a_denominator;
        let b = s1-a*x1;

        if a_nominator % a_denominator != 0 || b % x2 != 0 {
            continue;
        }

        let b = b/x2;
        part1 += a * 3 + b;
    }

    println!("{part1}");


    let mut part2 = 0;
    for d in data.iter() {
        let ((x1,y1),(x2,y2),(s1,s2)) = (d[0],d[1],d[2]);
        let s1 = s1 + 10000000000000;
        let s2 = s2 + 10000000000000;
        let a_nominator = s2*x2-s1*y2;
        let a_denominator = x2*y1-x1*y2;
        let a = a_nominator / a_denominator;
        let b = s1-a*x1;
        
        if a_nominator % a_denominator != 0 || b % x2 != 0 {
            continue;
        }
        
        let b = b/x2;
        part2 += a * 3 + b;
    }

    println!("{part2}");
}