mod hackerrank;

use hackerrank::task11::page_count;

fn main() {
    let n = 6;
    let p = 2;

    let result = page_count(n, p);

    println!("{}", result);
}
