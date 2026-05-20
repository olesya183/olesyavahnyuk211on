mod hackerrank;

use hackerrank::task06::get_total_x;

fn main() {
    let a = vec![2, 4];
    let b = vec![16, 32, 96];

    let result = get_total_x(a, b);

    println!("Result: {}", result);
}
