use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn pad((offset, state): &(i64, Vec<bool>)) -> (i64, Vec<bool>) {
    let first = state
        .iter()
        .enumerate()
        .find(|(_, &b)| b)
        .unwrap()
        .0;
    let last = state
        .iter()
        .enumerate()
        .filter(|(_, &b)| b)
        .last()
        .unwrap()
        .0;

    let padding = 5;
    let state: Vec<bool> = (0..padding)
        .map(|_| false)
        .chain(state.iter().skip(first).take(last - first + 1).cloned())
        .chain((0..padding).map(|_| false))
        .collect();
    (offset + first as i64 - padding, state)
}

fn step(map: &[bool], old: &(i64, Vec<bool>)) -> (i64, Vec<bool>) {
    let (off, state) = pad(old);
    let len = state.len();
    let mut newstate = vec![false; len];
    for j in 0..len - 5 {
        let idx: usize = (0..5).map(|k| (state[j + k] as usize) << k).sum();
        if map[idx] {
            newstate[j + 2] = true;
        }
    }
    (off, newstate)
}

fn eval((off, state): &(i64, Vec<bool>)) -> i64 {
    state
        .iter()
        .enumerate()
        .map(|(i, &x)| if x { i as i64 + off } else { 0 })
        .sum()
}

fn main() {
    let f = File::open("input").unwrap();
    let f = BufReader::new(&f);
    let mut lines = f.lines();

    let init: Vec<bool> = lines.next().unwrap().unwrap()[15..]
        .chars()
        .map(|c| c == '#')
        .collect();
    let mut map = [false; 32];

    for l in lines.skip(1) {
        let l = l.unwrap();
        let x: usize = l
            .chars()
            .take(5)
            .enumerate()
            .map(|(i, c)| {
                let b = (c == '#') as usize;
                b << i
            })
            .sum();
        let p = l.as_bytes()[9] == b'#';
        map[x] = p;
    }

    let mut state = (0, init.clone());
    for _ in 0..20 {
        state = step(&map, &state);
    }
    let r = eval(&state);
    println!("{}", r);

    let mut state = (0, init);
    let mut i = 0;
    let offoff;

    loop {
        let newstate = step(&map, &state);
        if state.1 == newstate.1 {
            offoff = newstate.0 - state.0;
            break;
        }
        state = newstate;
        i += 1;
    }
    let base = eval(&state);
    let ones = state.1.iter().filter(|&&b| b).count() as i64;
    let inc = offoff * ones;

    let miss = 50_000_000_000i64 - i;
    println!("{}", base + miss * inc);
}
