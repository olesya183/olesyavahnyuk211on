mod hackerrank;

fn main() {
    let result = hackerrank::task02::staircase(4);

    for line in result {
        println!("{}", line);
    }
}