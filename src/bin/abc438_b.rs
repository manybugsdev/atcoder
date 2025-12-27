fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut it = stdin.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();
    let s = it.next().unwrap();
    let t = it.next().unwrap();
    let mut min_count = usize::MAX;
    for i in 0..(n - m + 1) {
        let mut count = 0;
        let sbs = &s[i..].as_bytes();
        let tbs = &t.as_bytes();
        for j in 0..m {
            let sn = sbs[j] as usize - '0' as usize;
            let tn = tbs[j] as usize - '0' as usize;
            count += if sn >= tn { sn - tn } else { 10 + sn - tn }
        }
        if min_count > count {
            min_count = count
        }
    }
    println!("{}", min_count);
}
