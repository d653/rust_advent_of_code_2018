use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
#[macro_use]
extern crate text_io;
use itertools::Itertools;
use std::collections::HashMap;

struct Event {
    _mo: usize,
    _d: usize,
    _h: usize,
    m: usize,
    e: Evt,
}

enum Evt {
    BEGIN(usize),
    SLEEP,
    WAKE,
}

fn solve(v: &[Event]) {
    let mut time = HashMap::new();
    let mut sums = HashMap::new();
    let mut guard = 0;
    let mut sleep = 0;

    for ev in v {
        match ev.e {
            Evt::BEGIN(x) => {
                guard = x;
            }
            Evt::SLEEP => {
                sleep = ev.m;
            }
            Evt::WAKE => {
                let v = time.entry(guard).or_insert_with(||vec![0; 60]);
                //keep track of the sum of the sleeping time of each guard
                *sums.entry(guard).or_insert(0) += ev.m - sleep;
                //and the number of times a guard is sleeping in a specific minute
                for it in &mut v[sleep..ev.m] {
                    *it += 1;
                }
            }
        }
    }

    //given an array containing the sleeping time for each minute, returns (i,s), where s is the maximum, and i is its position
    let max = |v: &Vec<usize>| {
        v.iter()
            .cloned()
            .enumerate()
            .rev()
            .max_by_key(|&(_i, x)| x)
            .unwrap()
    };

    //find the guard with the highest sum
    let (g, _s) = sums.iter().max_by_key(|&(_, s)| s).unwrap();
    println!("{}", g * max(&time[&g]).0);

    let (g, (i, _s)) = time
        .iter()
        //find the pair (i,s) of each guard
        .map(|(&g, v)| (g, max(v)))
        //maximize by s
        .max_by_key(|&(_, (i, s))| (s, -(i as isize)))
        .unwrap();
    println!("{}", g * i);
}

fn main() {
    let f = File::open("input").unwrap();
    let f = BufReader::new(&f);
    let v = f.lines().map(|l| l.unwrap()).sorted();

    let v : Vec<_> = v
        .iter()
        .map(|l| {
            let (mo, d, h, m);
            let ev: String;
            scan!(l.bytes() => "[1518-{}-{} {}:{}] {}\n", mo,d,h,m,ev);
            let e = match ev.chars().next().unwrap() {
                'G' => {
                    let g;
                    scan!(ev.bytes() => "Guard #{} begins shift",g);
                    Evt::BEGIN(g)
                }
                'f' => Evt::SLEEP,
                'w' => Evt::WAKE,
                _ => unreachable!(),
            };
            Event {
                _mo: mo,
                _d: d,
                _h: h,
                m,
                e,
            }
        })
        .collect();

    solve(&v);
}
