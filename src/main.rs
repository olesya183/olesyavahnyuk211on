mod hackerrank;

use hackerrank::task11::page_count;
use std::io;

fn main() {
    let mut n_input = String::new();
    io::stdin().read_line(&mut n_input).unwrap();
    let n: i32 = n_input.trim().parse().unwrap();

    let mut p_input = String::new();
    io::stdin().read_line(&mut p_input).unwrap();
    let p: i32 = p_input.trim().parse().unwrap();

    let result = page_count(n, p);

    println!("{}", result);
}