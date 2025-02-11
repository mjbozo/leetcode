// LeetCode problem 1910: Remove All Occurrences of a Substring
// https://leetcode.com/problems/remove-all-occurrences-of-a-substring/description/

fn main() {
    let result = remove_occurrences(String::from("daabcbaabcbc"), String::from("abc"));
    println!("Result = {result}");
}

fn remove_occurrences(s: String, part: String) -> String {
    let p = part.len();
    let mut stack = vec![];

    for c in s.bytes() {
        stack.push(c);
        let n = stack.len();
        if n >= p {
            let stack_end = &stack[n - p..n];
            if chars_equal(stack_end, part.bytes().collect()) {
                for _ in 0..p {
                    stack.pop();
                }
            }
        }
    }

    return String::from_utf8(stack).unwrap();
}

fn chars_equal(stack_end: &[u8], part: Vec<u8>) -> bool {
    for i in 0..part.len() {
        if stack_end[i] != part[i] {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = remove_occurrences(String::from("daabcbaabcbc"), String::from("abc"));
        assert_eq!(result, String::from("dab"));
    }

    #[test]
    fn example_two() {
        let result = remove_occurrences(String::from("axxxxyyyyb"), String::from("xy"));
        assert_eq!(result, String::from("ab"));
    }
}
