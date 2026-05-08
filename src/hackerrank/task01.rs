pub fn staircase(n: i32) -> Vec<String> {
    let mut result = Vec::new();

    for i in 1..=n {
        let spaces = (n - i) as usize;
        let hashes = i as usize;

        let line = " ".repeat(spaces) + &"#".repeat(hashes);
        result.push(line);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staircase_4() {
        let result = staircase(4);

        let expected = vec![
            "   #".to_string(),
            "  ##".to_string(),
            " ###".to_string(),
            "####".to_string(),
        ];

        assert_eq!(result, expected);
    }
}