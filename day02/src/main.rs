use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use itertools::Itertools;
use std::iter::once;

fn part1(v: &Vec<String>) {
    //compute the frequency of each character
    let freq = |s: &String| {
        s.chars().fold(HashMap::new(), |mut h, c| {
            *h.entry(c).or_insert(0) += 1;
            h
        })
    };

    //produces a pair of booleans telling if there are pairs or triples
    let has23 = |f: HashMap<_, _>| {
        f.values()
            .fold((false, false), |(a, b), &c| (a || c == 2, b || c == 3))
    };

    //pairwise sum
    let sum2 = |(a, b), (c, d)| (a + c, b + d);

    let (r2, r3) = v
        .iter()
        .map(|x| has23(freq(x)))
        .map(|(a, b)| (a as i32, b as i32))
        .fold((0, 0), sum2);

    println!("{}", r2 * r3);
}

fn part2(v: &Vec<String>) {

    //something similar to a rolling hash (it is even simpler, we just add things)
    let add = |h:&mut u32,c:char|{
        *h = h.wrapping_mul(17);
        *h = h.wrapping_add(c as u32);
        *h
    };

    //produce a hash for each possible substring 0..i, in linear time
    let hashes = |it : &mut Iterator<Item=char>|{
        let mut h = 0;
        once(0).chain(it.map(|c|add(&mut h,c))).collect_vec()
    };

    let len = v[0].len();

    let mut h = HashMap::new();
    for x in v {
        //hashes from the beginning of the string to some position i
        let h1 = hashes(&mut x.chars());
        //hashes from the end of the strings to some position j
        let h2 = hashes(&mut x.chars().rev());
        for i in 0..len {
            //the new hash is composed by the hashes of the two strings 0..i and i+1..len, and i
            let hash = (h1[i],i,h2[len-i-1]);
            h.entry(hash).or_insert(vec![]).push((i,x));
        }
    }

    for v in h.values() {
        // if no collisions happened, v.len() is always 1, except for when we find the solution, in that case it will be 2
        // if there are collisions there is just slightly more work to do, the solution is still fine
        // there are no collisions with my AoC input
        for i in 0..v.len() {
            for j in i+1..v.len() {
                let (k1,s1) = v[i];
                let (k2,s2) = v[j];
                if k1 == k2 && s1[..k1] == s2[..k1] && s1[k1+1..] == s2[k1+1..] {
                    println!("{}{}",&s1[..k1],&s1[k1+1..]);
                }
            }
        }
    }

}

fn main() {
    let f = File::open("input").unwrap();
    let f = BufReader::new(&f);

    let v = f.lines().map(|l| l.unwrap()).collect();

    part1(&v);
    part2(&v);
}
