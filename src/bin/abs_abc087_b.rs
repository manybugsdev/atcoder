fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let [a, b, c, x] = stdin
        .split_whitespace()
        .map(|c| c.parse::<i64>().unwrap())
        .collect::<Vec<_>>()[..]
    else {
        panic!()
    };
    let mut count = 0;
    for ai in (0..=a).rev() {
        if 500 * ai > x {
            continue;
        }
        for bi in (0..=b).rev() {
            if 500 * ai + 100 * bi > x {
                continue;
            }
            for ci in (0..=c).rev() {
                let v = 500 * ai + 100 * bi + 50 * ci;
                if v >= x {
                    if v == x {
                        count += 1
                    }
                    continue;
                }
                break;
            }
        }
    }
    println!("{count}")
}
