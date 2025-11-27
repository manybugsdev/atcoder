use std::collections::*;
use std::io::*;

fn main() {
    let alphabets: Vec<char> = (0..26).map(|v| char::from_u32(v + 65).unwrap()).collect();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let [n, q] = input.split_whitespace().collect::<Vec<_>>()[..] else {
        panic!()
    };
    let [n, q] = [n, q].map(|v| v.parse::<usize>().unwrap());
    let mut list: Vec<char> = Vec::new();
    for i in 0..n {
        let a = alphabets[i];
        let mut left = 0;
        let mut right = list.len() - 1;
        while left <= right {
            let mid = (left + right / 2);
            let b = list[mid];
            print!("? {a} {b}");
            stdout().flush().unwrap();
            stdin().read_line(&mut input).unwrap();
            if input == ">" {
                left = mid;
            } else {
                right = mid
            }
        }
    }
}
