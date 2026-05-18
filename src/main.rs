mod hackerrank;

use hackerrank::task04::count_apples_and_oranges;

fn main() {

    let s = 7;
    let t = 11;

    let a = 5;
    let b = 15;

    let apples = vec![-2, 2, 1];
    let oranges = vec![5, -6];

    let result = count_apples_and_oranges(
        s,
        t,
        a,
        b,
        apples,
        oranges,
    );

    println!("Apples: {}", result.0);
    println!("Oranges: {}", result.1);
}