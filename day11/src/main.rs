use itertools::Itertools;

const SZ: usize = 300;

fn best(pfix: &Vec<Vec<i32>>, dim: usize) -> (i32, usize, usize, usize) {
    let mut best = -(SZ as i32 * SZ as i32 * 10);
    let mut p = (0, 0);
    for i in dim..=SZ {
        for j in dim..=SZ {
            let cur = pfix[i][j] - pfix[i - dim][j] - pfix[i][j - dim] + pfix[i - dim][j - dim];
            if cur > best {
                best = cur;
                p = (j - dim, i - dim);
            }
        }
    }
    (best, p.0, p.1, dim)
}

fn main() {
    let serial = 4842;
    let pl = |x: usize, y: usize| {
        let rid = x + 10;
        let pl = (rid * y + serial) * rid;
        (pl % 1000 / 100) as i32 - 5
    };

    let mut pls = vec![vec![0; 1 + SZ]; 1 + SZ];

    for i in 0..SZ {
        for j in 0..SZ {
            pls[i + 1][j + 1] = pl(j, i);
        }
    }

    let mut pfix = pls.clone();
    for i in 1..=SZ {
        for j in 1..=SZ {
            pfix[i][j] += pfix[i][j - 1];
        }
    }
    for i in 1..=SZ {
        for j in 1..=SZ {
            pfix[i][j] += pfix[i - 1][j];
        }
    }

    let sol3 = best(&pfix, 3);
    println!("{},{}", sol3.1, sol3.2);

    let sol = (0..SZ).map(|i| best(&pfix, i)).max_by_key(|x| x.0).unwrap();
    println!("{},{},{}", sol.1, sol.2, sol.3);
}
