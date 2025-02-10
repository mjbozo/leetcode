// LeetCode problem 3174: Clear Digits
// https://leetcode.com/problems/clear-digits/description/

fn main() {
    let result = clear_digits("abc".into());
    println!("Result = {result}");
}

fn clear_digits(s: String) -> String {
    let mut stack = vec![];
    for c in s.chars() {
        match c {
            '0'..='9' => {
                stack.pop();
            }
            _ => {
                stack.push(c);
            }
        };
    }
    return stack.into_iter().collect();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = clear_digits("abc".into());
        assert_eq!(result, String::from("abc"));
    }

    #[test]
    fn example_two() {
        let result = clear_digits("cb34".into());
        assert_eq!(result, String::new());
    }
}
