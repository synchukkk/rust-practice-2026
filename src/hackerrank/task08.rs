fn breaking_records(scores: &[i32]) -> (i32, i32) {
    let mut min = scores[0];
    let mut max = scores[0];
    let mut min_breaks = 0;
    let mut max_breaks = 0;

    for &s in &scores[1..] {
        if s > max {
            max = s;
            max_breaks += 1;
        } else if s < min {
            min = s;
            min_breaks += 1;
        }
    }
    (max_breaks, min_breaks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(breaking_records(&[10, 5, 20, 20, 4, 5, 2, 25, 1]), (2, 4));
    }

    #[test]
    fn test_example_2() {
        assert_eq!(breaking_records(&[3, 4, 21, 36, 10, 28, 35, 5, 24, 42]), (4, 0));
    }

    #[test]
    fn test_single_element() {
        assert_eq!(breaking_records(&[5]), (0, 0));
    }
}
