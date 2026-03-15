fn grading_students(grades: &[i32]) -> Vec<i32> {
    grades.iter().map(|&g| round_grade(g)).collect()
}

fn round_grade(grade: i32) -> i32 {
    if grade < 38 {
        return grade;
    }
    let next_multiple = ((grade / 5) + 1) * 5;
    if next_multiple - grade < 3 {
        next_multiple
    } else {
        grade
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grading_students() {
        let input = vec![73, 67, 38, 33];
        let expected = vec![75, 67, 40, 33];
        assert_eq!(grading_students(&input), expected);
    }

    #[test]
    fn test_below_38() {
        assert_eq!(round_grade(35), 35);
        assert_eq!(round_grade(37), 37);
    }

    #[test]
    fn test_rounding() {
        assert_eq!(round_grade(73), 75);
        assert_eq!(round_grade(67), 67);
        assert_eq!(round_grade(38), 40);
    }
}
