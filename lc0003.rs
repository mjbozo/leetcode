// LeetCode problem 0003: Longest Substring Without Repeating Characters
// https://leetcode.com/problems/longest-substring-without-repeating-characters/description/

use std::collections::HashSet;

fn main() {
    let result = length_of_longest_substring(String::from("abcabcbb"));
    println!("Result = {result}");
}

fn length_of_longest_substring(s: String) -> i32 {
    let mut index = 0;
    let mut window_size = 1;

    if s.len() == 0 || s.len() == 1 {
        return s.len() as i32;
    }

    while index + window_size <= s.len() {
        let substring = &s[index..index+window_size];
        let mut unique_chars = HashSet::new();
        let mut all_chars_unique = true;
        for char in substring.chars() {
            if unique_chars.contains(&char) {
                all_chars_unique = false;
            } else {
                unique_chars.insert(char);
            }
        }

        if all_chars_unique {
            window_size += 1;
        } else {
            index += 1;
        }
    }

    return (window_size - 1) as i32;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = length_of_longest_substring(String::from("abcabcbb"));
        assert_eq!(result, 3);
    }

    #[test]
    fn example_two() {
        let result = length_of_longest_substring(String::from("bbbbb"));
        assert_eq!(result, 1);
    }

    #[test]
    fn example_three() {
        let result = length_of_longest_substring(String::from("pwwkew"));
        assert_eq!(result, 3);
    }
}
