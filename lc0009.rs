// LeetCode problem 0009: Palindrome Number
// https://leetcode.com/problems/palindrome-number/description/

fn main() {
    let result = is_palindrome(121);
    println!("Result = {result}");
}

fn is_palindrome(x: i32) -> bool {
    let num_string = x.to_string();
    return num_string == num_string.chars().rev().collect::<String>();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = is_palindrome(121);
        assert!(result);
    }

    #[test]
    fn example_two() {
        let result = is_palindrome(-121);
        assert!(!result);
    }

    #[test]
    fn example_three() {
        let result = is_palindrome(10);
        assert!(!result);
    }
}
