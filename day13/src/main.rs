use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let f = File::open("input").unwrap();
    let f = BufReader::new(&f);
    let lines = f.lines();

    let mut m: Vec<Vec<char>> = lines.map(|l| l.unwrap().chars().collect()).collect();
    let mut carts = vec![];

    for (y, v) in m.iter_mut().enumerate() {
        for (x, c) in v.iter_mut().enumerate() {
            let cart = *c;
            let (nc, add) = match cart {
                '^' => ('|', true),
                'v' => ('|', true),
                '>' => ('-', true),
                '<' => ('-', true),
                c => (c, false),
            };
            *c = nc;
            let cs = ['^', 'v', '>', '<'];
            if add {
                let cart = cs.iter().position(|&c| c == cart).unwrap();
                carts.push((y as i32, x as i32, cart, 0));
            }
        }
    }

    while carts.len() > 1 {
        let mut dead = vec![false; carts.len()];
        let mut pos: HashMap<_, _> = carts
            .iter()
            .enumerate()
            .map(|(i, (y, x, _, _))| ((*y, *x), i))
            .collect();

        for (i, cart) in carts.iter_mut().enumerate() {
            let (y, x, c, t) = *cart;
            let offsets = [(-1, 0), (1, 0), (0, 1), (0, -1)];
            let (oy, ox) = offsets[c];
            let (ny, nx) = (y + oy, x + ox);
            let turns = [[2, 3, 0, 1], [3, 2, 1, 0]];
            let cross = [[3, 2, 0, 1], [0, 1, 2, 3], [2, 3, 1, 0]];
            let mut nt = t;

            let nc = match m[ny as usize][nx as usize] {
                '|' => c,
                '-' => c,
                '/' => turns[0][c],
                '\\' => turns[1][c],
                '+' => {
                    nt = (t + 1) % 3;
                    cross[t][c]
                }
                _ => unreachable!(),
            };

            pos.remove(&(y, x));
            if pos.contains_key(&(ny, nx)) {
                println!("crash {},{}", nx, ny);
                dead[pos[&(ny, nx)]] = true;
                dead[i] = true;
            } else {
                pos.insert((ny, nx), i);
            }
            *cart = (ny, nx, nc, nt);
        }

        carts = carts
            .into_iter()
            .enumerate()
            .filter(|&(i, _)| !dead[i])
            .map(|(_, c)| c)
            .collect();
        carts.sort();
    }

    println!("remaining {},{}", carts[0].1, carts[0].0);
}
