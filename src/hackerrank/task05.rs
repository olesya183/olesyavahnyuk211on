// https://www.hackerrank.com/challenges/kangaroo/problem

pub fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 <= v2 {
        return String::from("NO");
    }

    let mut first_position = x1;
    let mut second_position = x2;

    while first_position <= second_position {
        if first_position == second_position {
            return String::from("YES");
        }

        first_position += v1;
        second_position += v2;
    }

    String::from("NO")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kangaroo_yes() {
        let result = kangaroo(0, 3, 4, 2);

        assert_eq!(result, "YES");
    }

    #[test]
    fn test_kangaroo_no() {
        let result = kangaroo(0, 2, 5, 3);

        assert_eq!(result, "NO");
    }
}
