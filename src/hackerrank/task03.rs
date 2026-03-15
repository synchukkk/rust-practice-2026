fn staircase(n: i32) {
    for i in 1..=n {
        let spaces = n - i;
        let hashes = i;
        println!("{}{}", " ".repeat(spaces as usize), "#".repeat(hashes as usize));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staircase_4() {
        // n=4:
        //    #
        //   ##
        //  ###
        // ####
        staircase(4);
    }

    #[test]
    fn test_staircase_1() {
        staircase(1);
    }

    #[test]
    fn test_staircase_6() {
        staircase(6);
    }
}
