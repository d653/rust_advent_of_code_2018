
fn main() {
    let x = 330121;
    let vx = [3, 3, 0, 1, 2, 1];

    let mut v = vec![3, 7];
    let (mut i, mut j) = (0, 1);
    let mut sol1 = false;
    let mut sol2 = false;

    while !sol1 || !sol2 {
        let s = v[i] + v[j];
        if s < 10 {
            v.push(s);
        } else {
            v.push(s / 10);
            v.push(s % 10);
        }
        i = (i + 1 + v[i]) % v.len();
        j = (j + 1 + v[j]) % v.len();

        if !sol1 && v.len() >= x + 10 {
            println!(
                "{}",
                v[x..x + 10]
                    .iter()
                    .map(|&c| (b'0' + c as u8) as char)
                    .collect::<String>()
            );
            sol1 = true;
        }

        let len = v.len();
        let ilen = vx.len();
        if !sol2 && len > ilen {
            if let Some(p) = v[len - ilen - 1..].windows(ilen).position(|x| x == vx) {
                println!("{}", len - ilen - 1 + p);
                sol2 = true;
            }
        }
    }
}
