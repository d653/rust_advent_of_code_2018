use itertools::Itertools;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn dist((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn bb(v: &[(i32, i32)]) -> (i32, i32, i32, i32) {
    let (minx, maxx) = v.iter().map(|(x, _)| x).minmax().into_option().unwrap();
    let (miny, maxy) = v.iter().map(|(_, y)| y).minmax().into_option().unwrap();
    (*minx, *maxx, *miny, *maxy)
}

fn part1(v: &[(i32, i32)]) {
    let (minx, maxx, miny, maxy) = bb(v);

    let ysz = (maxy - miny + 1) as usize;
    let xsz = (maxx - minx + 1) as usize;

    let nearest = |(x1, y1)| {
        let min1 = v
            .iter()
            .enumerate()
            .min_by_key(|(_, &(x2, y2))| dist((x1, y1), (x2, y2)))
            .unwrap()
            .0;
        let min2 = v
            .iter()
            .enumerate()
            .rev()
            .min_by_key(|(_, &(x2, y2))| dist((x1, y1), (x2, y2)))
            .unwrap()
            .0;
        if min1 == min2 {
            Some(min1)
        } else {
            None
        }
    };

    let mut b = vec![false; v.len()];
    let mut c = vec![0; v.len()];

    for i in 0..ysz {
        for j in 0..xsz {
            if let Some(p) = nearest((minx + j as i32, miny + i as i32)) {
                if i == 0 || j == 0 || i == ysz - 1 || j == xsz - 1 {
                    b[p] = true;
                }
                c[p] += 1;
            }
        }
    }

    let (_, c) = c
        .iter()
        .enumerate()
        .filter(|&(i, _)| !b[i])
        .max_by_key(|&(_, c)| c)
        .unwrap();
    println!("{}", c);
}

fn part2(v: &[(i32, i32)]) {
    let (minx, maxx, miny, maxy) = bb(v);
    let tsh = 10000;
    let add = 100;
    let (minx, maxx, miny, maxy) = (minx - add, maxx + add, miny - add, maxy + add);
    let mut c = 0;

    for i in miny..=maxy {
        for j in minx..=maxx {
            let s: i32 = v.iter().map(|&p| dist(p, (j, i))).sum();
            if s < tsh {
                c += 1;
            }
        }
    }
    println!("{}", c);
}

fn main() {
    let f = File::open("input").unwrap();
    let f = BufReader::new(&f);
    let v: Vec<(i32, i32)> = f
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let mut it = l.split(", ").map(|x| x.parse().unwrap());
            (it.next().unwrap(), it.next().unwrap())
        })
        .collect();

    part1(&v);
    part2(&v);
}
