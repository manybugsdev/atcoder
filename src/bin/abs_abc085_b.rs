use std::collections::*;

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut sw = stdin.split_whitespace();
    sw.next();
    let mochis = sw
        .map(|c| c.parse::<i64>().unwrap())
        .collect::<HashSet<_>>();
    println!("{}", mochis.len())
}
