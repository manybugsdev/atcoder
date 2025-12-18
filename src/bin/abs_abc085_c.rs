fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut it = stdin
        .split_whitespace()
        .map(|v| v.parse::<isize>().unwrap());
    let n = it.next().unwrap();
    let y = it.next().unwrap();
    for a in 0..=n {
        for b in 0..=(n - a) {
            let c = n - a - b;
            if 10000 * a + 5000 * b + 1000 * c == y {
                println!("{} {} {}", a, b, c);
                return;
            }
        }
    }
    println!("-1 -1 -1");
}
