use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn part1(v: &Vec<i32>) {
    let s: i32 = v.iter().sum();

    println!("{}", s);
}

fn part2(v: &Vec<i32>) {
    let mut freq = 0;
    let mut seen = HashSet::new();
    seen.insert(freq);

    v.iter().cycle().all(|x| {
        freq += x;
        seen.insert(freq)
    });

    println!("{}", freq);
}

fn main() {
    let f = File::open("input").unwrap();
    let f = BufReader::new(&f);

    let v = f.lines().map(|l| l.unwrap().parse().unwrap()).collect();

    part1(&v);
    part2(&v);
}
