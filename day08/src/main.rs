use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
struct Node {
    children: Vec<Box<Node>>,
    metadata: Vec<usize>,
}

fn parse(v: &[usize]) -> (Node, usize) {
    let nc = v[0];
    let nm = v[1] as usize;

    let mut children = vec![];
    let mut p = 2;
    for _ in 0..nc {
        let (n, i) = parse(&v[p..]);
        children.push(Box::new(n));
        p += i;
    }

    let metadata = v[p..p + nm].to_vec();
    (Node { children, metadata }, p + nm)
}

fn sum1(node: &Node) -> usize {
    let meta: usize = node.metadata.iter().sum();
    let desc: usize = node.children.iter().map(|c| sum1(c)).sum();
    meta + desc
}

fn sum2(node: &Node) -> usize {
    if node.children.is_empty() {
        node.metadata.iter().sum()
    } else {
        let v: Vec<_> = node.children.iter().map(|c| sum2(c)).collect();
        node.metadata
            .iter()
            .map(|&m| v.get(m - 1).unwrap_or(&0))
            .sum()
    }
}

fn solve(v: &[usize]) {
    let tree = parse(v).0;
    println!("{}", sum1(&tree));
    println!("{}", sum2(&tree));
}

fn main() {
    let f = File::open("input").unwrap();
    let f = BufReader::new(&f);
    let s = f.lines().next().unwrap().unwrap();
    let v: Vec<_> = s.split(' ').map(|w| w.parse().unwrap()).collect();

    solve(&v);
}
