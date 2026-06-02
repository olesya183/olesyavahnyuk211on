//https://www.hackerrank.com/challenges/diagonal-difference/problem
pub fn diagonal_difference(arr: Vec<Vec<i32>>) -> i32 {
    let n = arr.len();

    let mut sum1 = 0;
    let mut sum2 = 0;

    for i in 0..n {
        for j in 0..n {
            if i == j {
                sum1 += arr[i][j];
            }

            if i + j == n - 1 {
                sum2 += arr[i][j];
            }
        }
    }

    (sum1 - sum2).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagonal_difference() {
        let arr = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![9, 8, 9],
        ];

        assert_eq!(diagonal_difference(arr), 2);
    }
}