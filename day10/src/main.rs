use itertools::Itertools;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn solve(v: &Vec<((i32, i32), (i32, i32))>) {
    let vf: Vec<_> = v
        .iter()
        .map(|&((a, b), (c, d))| move |i: i32| (a + i * c, b + i * d))
        .collect();
    let ((x0, y0), (vx0, vy0)) = v[0];
    let ((x1, y1), (vx1, vy1)) = *v
        .iter()
        .find(|(_, (vx1, vy1))| *vx1 != vx0 || *vy1 != vy0)
        .unwrap();

    let m0 = vy0 as f32 / vx0 as f32;
    let m1 = vy1 as f32 / vx1 as f32;
    let x = (y1 as f32 - y0 as f32 + m0 * x0 as f32 - m1 * x1 as f32) / (m1 - m0);
    let y = m0 * (x - x0 as f32) + y0 as f32;
    let k = ((y - y0 as f32) / vy0 as f32) as i32;

    let (best, _) = (k - 100..k + 100)
        .map(|i| {
            let (miny, maxy) = vf.iter().map(|f| f(i).1).minmax().into_option().unwrap();
            (i, maxy - miny)
        })
        .min_by_key(|x| x.1)
        .unwrap();

    let (miny, maxy) = vf.iter().map(|f| f(best).1).minmax().into_option().unwrap();
    let (minx, maxx) = vf.iter().map(|f| f(best).0).minmax().into_option().unwrap();
    let mut m = vec![vec![' '; (maxx - minx + 1) as usize]; (maxy - miny + 1) as usize];
    for f in vf {
        let (x, y) = f(best);
        m[(y - miny) as usize][(x - minx) as usize] = '#';
    }
    for v in m {
        for x in v {
            print!("{}", x);
        }
        println!();
    }
    println!("{}", best);
}

fn main() {
    let f = File::open("input").unwrap();
    let f = BufReader::new(&f);

    let re = regex::Regex::new(
        r"position=<\s*([-]?\d*),\s*([-]?\d*)> velocity=<\s*([-]?\d*),\s*([-]?\d*)>",
    )
    .unwrap();

    let v: Vec<_> = f
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let v = re.captures_iter(&l).next().unwrap();
            (
                (v[1].parse().unwrap(), v[2].parse().unwrap()),
                (v[3].parse().unwrap(), v[4].parse().unwrap()),
            )
        })
        .collect();

    solve(&v);
}
