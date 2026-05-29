mod hackerrank;

use hackerrank::task08::migratory_birds;

fn main() {
    let arr = vec![1, 4, 4, 4, 5, 3];

    let result = migratory_birds(&arr);

    println!("Result: {}", result);
}

