fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut sw = stdin.split_whitespace();
    let n = sw.next().unwrap().parse::<usize>().unwrap();
    let s = sw.next().unwrap();
    println!("{}{}", "o".repeat(n - s.len()), s);
}
