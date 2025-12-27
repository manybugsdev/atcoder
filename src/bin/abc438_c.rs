fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut it = stdin
        .split_whitespace()
        .map(|v| v.parse::<usize>().unwrap());
    let n = it.next().unwrap();
    let mut a = it.collect::<Vec<_>>();
    'whole: loop {
        if a.len() < 4 {
            break;
        }
        for i in 0..(a.len() - 4) {
            if a[i] == a[i + 1] && a[i + 1] == a[i + 2] && a[i + 2] == a[i + 3] {
                a.drain(i..(i + 4));
                continue 'whole;
            }
        }
        break;
    }
    println!("{}", a.len());
}
