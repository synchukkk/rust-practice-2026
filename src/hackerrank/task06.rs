fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 == v2 {
        if x1 == x2 {
            "YES".to_string()
        } else {
            "NO".to_string()
        }
    } else if (x2 - x1) % (v1 - v2) == 0 && (x2 - x1) / (v1 - v2) > 0 {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yes() {
        assert_eq!(kangaroo(0, 3, 4, 2), "YES");
    }

    #[test]
    fn test_no() {
        assert_eq!(kangaroo(0, 2, 5, 3), "NO");
    }

    #[test]
    fn test_same_speed_same_position() {
        assert_eq!(kangaroo(0, 3, 0, 3), "YES");
    }

    #[test]
    fn test_same_speed_different_position() {
        assert_eq!(kangaroo(0, 3, 5, 3), "NO");
    }
}
