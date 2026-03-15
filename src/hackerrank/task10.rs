use std::collections::HashMap;

fn sock_merchant(ar: &[i32]) -> i32 {
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for &sock in ar {
        *counts.entry(sock).or_insert(0) += 1;
    }
    counts.values().map(|&v| v / 2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(sock_merchant(&[10, 20, 20, 10, 10, 30, 50, 10, 20]), 3);
    }

    #[test]
    fn test_all_pairs() {
        assert_eq!(sock_merchant(&[1, 1, 2, 2, 3, 3]), 3);
    }

    #[test]
    fn test_no_pairs() {
        assert_eq!(sock_merchant(&[1, 2, 3, 4, 5]), 0);
    }

    #[test]
    fn test_one_type() {
        assert_eq!(sock_merchant(&[5, 5, 5, 5, 5]), 2);
    }
}
