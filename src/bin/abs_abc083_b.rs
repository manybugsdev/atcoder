fn sum_digits(n: usize) -> usize {
    if n < 10 {
        return n;
    }
    return n % 10 + sum_digits(n / 10);
}

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let [n, a, b] = stdin
        .split_whitespace()
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<_>>()[..]
    else {
        panic!()
    };
    let mut sum = 0;
    for ni in 1..=n {
        let sd = sum_digits(ni);
        if a <= sd && sd <= b {
            sum += ni;
        }
    }
    println!("{sum}");
}
