fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut sw = stdin.split_whitespace();
    let n = sw.next().unwrap().parse::<usize>().unwrap();
    let mut list = sw.map(|v| v.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let mut count = 0;
    'outer: loop {
        for i in 0..n {
            if list[i] % 2 == 1 {
                break 'outer;
            }
            list[i] /= 2
        }
        count += 1
    }
    println!("{count}")
}
