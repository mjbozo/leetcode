// LeetCode problem 2108: Find First Palindromic String in the Array
// https://leetcode.com/problems/find-first-palindromic-string-in-the-array/description/

fn main() {
    let result = first_palindrome(vec!["abc".to_string(), "car".to_string(), "ada".to_string(), "racecar".to_string(), "cool".to_string()]);
    println!("Result = {result}");
}

fn first_palindrome(words: Vec<String>) -> String {
    for word in words {
        let mut right = word.len() / 2;
        let mut left = word.len() - right - 1;

        let bytes = word.as_bytes();
        while bytes[left] == bytes[right] {
            if left == 0 {
                return word;
            }
            left -= 1;
            right += 1;
        }
    }

    return String::new();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = first_palindrome(vec!["abc".to_string(), "car".to_string(), "ada".to_string(), "racecar".to_string(), "cool".to_string()]);
        assert_eq!(result, "ada".to_string());
    }

    #[test]
    fn example_two() {
        let result = first_palindrome(vec!["notapalindrome".to_string(), "racecar".to_string()]);
        assert_eq!(result, "racecar".to_string());
    }

    #[test]
    fn example_three() {
        let result = first_palindrome(vec!["def".to_string(), "ghi".to_string()]);
        assert_eq!(result, "".to_string());
    }
}
