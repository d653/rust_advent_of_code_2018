use priority_queue::PriorityQueue;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Deps {
    fw: HashMap<char, Vec<char>>,
    pq: PriorityQueue<char, (i32, i32)>,
}

fn prepare(v: &[(char, char)]) -> Deps {
    let mut forward = HashMap::new();
    let mut backward = HashMap::new();

    for &(a, b) in v {
        forward.entry(b).or_insert_with(|| vec![]);
        forward.entry(a).or_insert_with(|| vec![]).push(b);
        backward.entry(a).or_insert(0);
        *backward.entry(b).or_insert(0) += 1;
    }

    let mut pq = PriorityQueue::new();
    for (k, v) in backward.into_iter() {
        pq.push(k, (-v, -(k as i32)));
    }
    Deps { fw: forward, pq }
}

fn part1(v: &[(char, char)]) {
    let Deps {
        fw: forward,
        mut pq,
    } = prepare(v);

    let mut s = String::new();
    while let Some((c, _)) = pq.pop() {
        s.push(c);
        for x in &forward[&c] {
            pq.change_priority_by(x, |(v, k)| (v + 1, k));
        }
    }
    println!("{}", s);
}

fn part2(v: &[(char, char)]) {
    let Deps {
        fw: forward,
        pq: mut jwait,
    } = prepare(v);

    let base = 61;
    let nworkers = 5;
    let duration = |c| c as u8 as i32 - 65 + base;
    let mut wready = (0..nworkers).collect::<Vec<_>>();
    let mut pending = PriorityQueue::new();

    let mut t = 0;
    loop {
        //start ready jobs on ready workers
        while let (Some(&w), Some((&c, (0, _)))) = (wready.last(), jwait.peek()) {
            jwait.pop();
            wready.pop();
            let newt = t + duration(c);
            pending.push((w, c), -newt);
        }

        //jump ahead on time
        if let Some((_, &nt)) = pending.peek() {
            t = -nt;
        } else {
            break;
        }

        //remove constraints of completed jobs
        while let Some((&(w, c), p)) = pending.peek() {
            if t != -p {
                break;
            }
            pending.pop();
            wready.push(w);
            for x in &forward[&c] {
                jwait.change_priority_by(x, |(v, k)| (v + 1, k));
            }
        }
    }
    println!("{}", t);
}

fn main() {
    let f = File::open("input").unwrap();
    let f = BufReader::new(&f);
    let v: Vec<(char, char)> = f
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let l = l.as_bytes();
            (l[5] as char, l[36] as char)
        })
        .collect();

    part1(&v);
    part2(&v);
}
