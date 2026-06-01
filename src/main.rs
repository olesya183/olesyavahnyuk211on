mod hackerrank;

use hackerrank::task10::diagonal_difference;

fn main() {
    let arr = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![9, 8, 9],
    ];

    let result = diagonal_difference(arr);

    println!("{}", result);
}