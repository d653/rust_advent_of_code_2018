#[macro_use]
extern crate defmac;

fn solve(p: usize, m: usize) {
    let mut circle = linked_list::LinkedList::new();
    circle.push_front(0);
    let mut score = vec![0; p];

    let mut cur = circle.cursor();
    cur.next();

    defmac!(forward => if cur.next().is_none() { cur.next(); } );
    defmac!(backward => if cur.prev().is_none() { cur.prev(); } );

    for i in 1..=m {
        if i % 23 != 0 {
            for _ in 0..2 {
                forward!();
            }
            cur.insert(i);
        } else {
            for _ in 0..7 {
                backward!();
            }
            score[i%p] += i + cur.remove().unwrap();
        }
    }
    let best = score.iter().max().unwrap();
    println!("{}", best);
}

fn main() {
    let p = 416;
    let m = 71975;
    solve(p, m);
    solve(p, m * 100);
}
