fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut count = 0;
    for ch in stdin.chars() {
        if ch == '1' {
            count += 1;
        }
    }
    println!("{count}")
}
