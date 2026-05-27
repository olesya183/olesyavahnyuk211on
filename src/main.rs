mod hackerrank;

use hackerrank::task07::breaking_records;

fn main() {
    let scores = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];

    let result = breaking_records(scores);

    println!("Best record breaks: {}", result[0]);
    println!("Worst record breaks: {}", result[1]);
}


