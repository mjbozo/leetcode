// LeetCode problem 0005: Longest Palindromic Substring
// https://leetcode.com/problems/longest-palindromic-substring/description/

fn main() {
    let result = longest_palindrome(String::from("babad"));
    println!("Result = {result}");
}

fn longest_palindrome(s: String) -> String {
    for i in (1..=s.len()).rev() {
        for substring in s.chars().collect::<Vec<char>>().windows(i) {
            if substring.iter().collect::<Vec<_>>() == substring.iter().rev().collect::<Vec<_>>() {
                return substring.iter().collect::<String>();
            }
        }
    }

    return String::new();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = longest_palindrome(String::from("babad"));
        assert_eq!(result, String::from("bab"));
    }

    #[test]
    fn example_two() {
        let result = longest_palindrome(String::from("cbbd"));
        assert_eq!(result, String::from("bb"));
    }
}

