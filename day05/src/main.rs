use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn part1(s: &str) -> usize {
    let mut v = vec![' '];
    for c in s.chars() {
        let last = *v.last().unwrap();
        if c != last && c.eq_ignore_ascii_case(&last) {
            v.pop();
        } else {
            v.push(c);
        }
    }
    v.len() - 1
}

fn part2(s: &str) {
    let r = (0..26)
        .map(|i| {
            part1(
                &s.chars()
                    .filter(|&c| (c as u8|32) != (b'a' + i))
                    .collect::<String>(),
            )
        })
        .min()
        .unwrap();
    println!("{}", r);
}

fn main() {
    let f = File::open("input").unwrap();
    let f = BufReader::new(&f);
    let s = f.lines().next().map(|l| l.unwrap()).unwrap();

    println!("{}", part1(&s));
    part2(&s);
}
