// LeetCode problem 1957: Delete Characters to Make Fancy String
// https://leetcode.com/problems/delete-characters-to-make-fancy-string/description/

fn main() {
    let result = make_fancy_string(String::from("leeetcode"));
    println!("Result = {result}");
}

fn make_fancy_string(s: String) -> String {
    let mut result = String::with_capacity(s.len());
    let mut streak = 0;
    let chars = s.chars().collect::<Vec<_>>();

    for (i, c) in chars.iter().enumerate() {
        if i > 0 && *c == chars[i - 1] {
            streak += 1;
        } else {
            streak = 1;
        }

        if streak < 3 {
            result.push(*c);
        }
    }

    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = make_fancy_string(String::from("leeetcode"));
        assert_eq!(result, String::from("leetcode"));
    }

    #[test]
    fn example_two() {
        let result = make_fancy_string(String::from("aaabaaaa"));
        assert_eq!(result, String::from("aabaa"));
    }

    #[test]
    fn example_three() {
        let result = make_fancy_string(String::from("aab"));
        assert_eq!(result, String::from("aab"));
    }
}
