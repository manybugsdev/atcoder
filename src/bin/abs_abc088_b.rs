enum Player {
    Alice,
    Bob,
}

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut list = stdin
        .split_whitespace()
        .map(|c| c.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    list.remove(0);
    let mut alice = 0;
    let mut bob = 0;
    let mut player = Player::Alice;
    list.sort();
    list.reverse();
    for card in list {
        player = match player {
            Player::Alice => {
                alice += card;
                Player::Bob
            }
            Player::Bob => {
                bob += card;
                Player::Alice
            }
        }
    }
    println!("{}", alice - bob);
}
