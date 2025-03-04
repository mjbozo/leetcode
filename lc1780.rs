// LeetCode problem 1780: Check if Number is a Sum of Powers of Three
// https://leetcode.com/problems/check-if-number-is-a-sum-of-powers-of-three/description/

fn main() {
    let result = check_powers_of_three(12);
    println!("Result = {result}");
}

fn check_powers_of_three(n: i32) -> bool {
    let mut n = n;
    while n > 0 {
        if n % 3 == 2 {
            return false;
        }
        n /= 3;
    }
    return true;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = check_powers_of_three(12);
        assert!(result);
    }

    #[test]
    fn example_two() {
        let result = check_powers_of_three(91);
        assert!(result);
    }

    #[test]
    fn example_three() {
        let result = check_powers_of_three(21);
        assert!(!result);
    }
}
