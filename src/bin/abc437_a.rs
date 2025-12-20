fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut sw = stdin.split_whitespace();
    let [a, b] = sw.map(|v| v.parse::<usize>().unwrap()).collect::<Vec<_>>()[..] else {
        panic!()
    };
    println!("{}", a * 12 + b);
}
