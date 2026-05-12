// https://www.hackerrank.com/challenges/grading/problem
pub fn grading_students(grades: Vec<i32>) -> Vec<i32> {

    let mut result = Vec::new();

    for mut grade in grades {

        if grade >= 38 {

            let remainder = grade % 5;

            let difference = 5 - remainder;

            if difference < 3 && remainder != 0 {

                grade += difference;
            }
        }

        result.push(grade);
    }


    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_grading_students() {

        let grades = vec![73, 67, 38, 33];

        let result = grading_students(grades);

        assert_eq!(result, vec![75, 67, 40, 33]);
    }
}