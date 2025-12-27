fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut it = stdin
        .split_whitespace()
        .map(|v| v.parse::<isize>().unwrap());
    let d = it.next().unwrap();
    let f = it.next().unwrap();
    let mut i = f - 1;
    while i < d {
        i += 7;
    }
    println!("{}", i - d + 1);
}
