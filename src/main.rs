mod hackerrank;

use hackerrank::task09::sock_merchant;

fn main() {
    let socks = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];

    let result = sock_merchant(socks);

    println!("Pairs: {}", result);
}
