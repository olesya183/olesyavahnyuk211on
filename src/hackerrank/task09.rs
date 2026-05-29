// https://www.hackerrank.com/challenges/sock-merchant/problem
pub fn sock_merchant(socks: Vec<i32>) -> i32 {
    let mut pairs = 0;

    let mut used = vec![false; socks.len()];

    for i in 0..socks.len() {
        if used[i] {
            continue;
        }

        for j in (i + 1)..socks.len() {
            if !used[j] && socks[i] == socks[j] {
                used[i] = true;
                used[j] = true;

                pairs += 1;

                break;
            }
        }
    }

    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sock_merchant() {
        let socks = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];

        let result = sock_merchant(socks);

        assert_eq!(result, 3);
    }
}