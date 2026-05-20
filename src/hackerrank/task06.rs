// https://www.hackerrank.com/challenges/between-two-sets/problem

pub fn get_total_x(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let start = *a.iter().max().unwrap();
    let end = *b.iter().min().unwrap();

    let mut count = 0;

    for x in start..=end {
        let mut valid = true;

        for value in &a {
            if x % value != 0 {
                valid = false;
                break;
            }
        }

        if valid {
            for value in &b {
                if value % x != 0 {
                    valid = false;
                    break;
                }
            }
        }

        if valid {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_x() {
        let a = vec![2, 4];
        let b = vec![16, 32, 96];

        assert_eq!(get_total_x(a, b), 3);
    }
}
