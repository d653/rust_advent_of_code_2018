use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
#[macro_use] extern crate text_io;

struct Rect{
    id: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize
}

fn solve(v: &[Rect]) {
    let mut m = [[0;1000];1000];
    for r in v {
        for row in &mut m[r.y..r.y+r.h] {
            for it in &mut row[r.x..r.x+r.w] {
                *it += 1;
            }
        }
    }

    let r : usize = m.iter().map(|v|v.iter().filter(|&&x|x>1).count()).sum();
    println!("{}",r);

    'outer: for r in v {
        for row in &m[r.y..r.y+r.h] {
            for it in &row[r.x..r.x+r.w] {
                if *it != 1 {
                    continue 'outer;
                }
            }
        }
        println!("{}",r.id);
    }
}

fn main() {
    let f = File::open("input").unwrap(); 
    let f = BufReader::new(&f);

    let v : Vec<_> = f.lines().map(|l|{
        let l = l.unwrap();
        let (id,x,y,w,h);
        scan!(l.bytes() => "#{} @ {},{}: {}x{}", id,x,y,w,h);
        Rect{id,x,y,w,h}
    }).collect();

    solve(&v);
}
