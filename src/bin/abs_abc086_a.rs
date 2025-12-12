fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let [a, b] = stdin
        .split_whitespace()
        .map(|c| c.parse::<i64>().unwrap())
        .collect::<Vec<_>>()[..]
    else {
        panic!()
    };
    if a * b % 2 == 0 {
        println!("Even")
    } else {
        println!("Odd")
    }
}
