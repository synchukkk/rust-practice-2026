fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: i32, b: i32) -> i32 {
    a / gcd(a, b) * b
}

fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let lcm_a = a.iter().fold(1, |acc, &x| lcm(acc, x));
    let gcd_b = b.iter().fold(b[0], |acc, &x| gcd(acc, x));

    let mut count = 0;
    let mut x = lcm_a;
    while x <= gcd_b {
        if gcd_b % x == 0 {
            count += 1;
        }
        x += lcm_a;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(get_total_x(&[2, 4], &[16, 32, 96]), 3);
    }

    #[test]
    fn test_single_elements() {
        assert_eq!(get_total_x(&[3], &[9]), 1);
    }

    #[test]
    fn test_no_result() {
        assert_eq!(get_total_x(&[3, 5], &[10]), 0);
    }
}
