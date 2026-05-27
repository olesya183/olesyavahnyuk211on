// https://www.hackerrank.com/challenges/breaking-best-and-worst-records/problem
pub fn breaking_records(scores: Vec<i32>) -> Vec<i32> {
    let mut best_score = scores[0];
    let mut worst_score = scores[0];

    let mut best_breaks = 0;
    let mut worst_breaks = 0;

    for score in scores {
        if score > best_score {
            best_score = score;
            best_breaks += 1;
        }

        if score < worst_score {
            worst_score = score;
            worst_breaks += 1;
        }
    }

    vec![best_breaks, worst_breaks]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breaking_records() {
        let scores = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];

        let result = breaking_records(scores);

        assert_eq!(result, vec![2, 4]);
    }
}