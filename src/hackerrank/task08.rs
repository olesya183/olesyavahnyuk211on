// https://www.hackerrank.com/challenges/migratory-birds/problem
pub fn migratory_birds(arr: &[i32]) -> i32 {
    let mut max_count = 0;
    let mut answer = 0;

    for i in 0..arr.len() {
        let mut count = 0;

        for j in 0..arr.len() {
            if arr[i] == arr[j] {
                count += 1;
            }
        }

        if count > max_count {
            max_count = count;
            answer = arr[i];
        }

        if count == max_count && arr[i] < answer {
            answer = arr[i];
        }
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migratory_birds() {
        let arr = vec![1, 4, 4, 4, 5, 3];

        let result = migratory_birds(&arr);

        assert_eq!(result, 4);
    }

    #[test]
    fn test_smallest_id() {
        let arr = vec![1, 2, 2, 3, 3];

        let result = migratory_birds(&arr);

        assert_eq!(result, 2);
    }
}