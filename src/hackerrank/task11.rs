//https://www.hackerrank.com/challenges/drawing-book

pub fn page_count(n: i32, p: i32) -> i32 {
    let front = p / 2;
    let back = (n - p) / 2;

    if front < back {
        front
    } else {
        back
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_count() {
        assert_eq!(page_count(6, 2), 1);
    }
}