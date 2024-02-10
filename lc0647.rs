// LeetCode problem 0647: Palindromic Substrings
// https://leetcode.com/problems/palindromic-substrings/description/

fn main() {
    let result = count_substrings(String::from("abc"));
    println!("Result = {result}");
}

fn count_substrings(s: String) -> i32 {
    let mut count = 0;
    let s = s.as_bytes();
    for i in 0..s.len() {
        count += expand(&s, i, i);
        count += expand(&s, i, i + 1);
    }
    return count;
}

fn expand(s: &[u8], mut left: usize, mut right: usize) -> i32 {
    let mut count = 0;
    while right < s.len() && s[left] == s[right] {
        count += 1;
        if left == 0 {
            break;
        }
        left -= 1;
        right += 1;
    }
    return count;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = count_substrings(String::from("abc"));
        assert_eq!(result, 3);
    }

    #[test]
    fn example_two() {
        let result = count_substrings(String::from("aaa"));
        assert_eq!(result, 6);
    }
}
