// LeetCode problem 0020: Valid Parentheses
// https://leetcode.com/problems/valid-parentheses/description/

use std::collections::VecDeque;

fn main() {
    let result = is_valid(String::from("()"));
    println!("Result = {result}");
}

fn is_valid(s: String) -> bool {
    let mut stack = VecDeque::new();
    let bytes = s.as_bytes();

    for byte in bytes {
        match byte {
            b'(' | b'{' | b'[' => stack.push_back(*byte),
            b')' => {
                if stack.pop_back() != Some(b'(') {
                    return false;
                }
            },
            b'}' => {
                if stack.pop_back() != Some(b'{') {
                    return false;
                }
            },
            b']' => {
                if stack.pop_back() != Some(b'[') {
                    return false;
                }
            },
            _ => return false
        }
    }

    return stack.is_empty();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = is_valid(String::from("()"));
        assert!(result);
    }

    #[test]
    fn example_two() {
        let result = is_valid(String::from("()[]{}"));
        assert!(result);
    }

    #[test]
    fn example_three() {
        let result = is_valid(String::from("(]"));
        assert!(!result);
    }
}
