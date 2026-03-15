fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) -> (usize, usize) {
    let apple_count = apples.iter().filter(|&&x| (a + x) >= s && (a + x) <= t).count();
    let orange_count = oranges.iter().filter(|&&x| (b + x) >= s && (b + x) <= t).count();
    (apple_count, orange_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let apples = vec![2, 3, -4];
        let oranges = vec![3, -2, -4];
        assert_eq!(count_apples_and_oranges(7, 11, 5, 15, &apples, &oranges), (1, 1));
    }

    #[test]
    fn test_no_fruits_in_range() {
        let apples = vec![-5, -6];
        let oranges = vec![5, 6];
        assert_eq!(count_apples_and_oranges(7, 11, 5, 15, &apples, &oranges), (0, 2));
    }
}
