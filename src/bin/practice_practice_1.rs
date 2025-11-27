use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let [a, b, c, s] = input.split_whitespace().collect::<Vec<_>>()[..] else {
        panic!()
    };
    let [a, b, c] = [a, b, c].map(|v| v.parse::<i64>().unwrap());
    print!("{} {}", a + b + c, s)
}
