use std::collections::HashMap;

fn migratory_birds(arr: &[i32]) -> i32 {
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for &bird in arr {
        *counts.entry(bird).or_insert(0) += 1;
    }
    let max_count = *counts.values().max().unwrap();
    let mut result: Vec<i32> = counts.iter()
        .filter(|(_, &v)| v == max_count)
        .map(|(&k, _)| k)
        .collect();
    result.sort();
    result[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(migratory_birds(&[1, 4, 4, 4, 5, 3]), 4);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(migratory_birds(&[1, 2, 3, 4, 5, 4, 3, 2, 1, 3, 4]), 3);
    }

    #[test]
    fn test_tie_returns_smallest() {
        assert_eq!(migratory_birds(&[1, 1, 2, 2, 3, 3]), 1);
    }
}
