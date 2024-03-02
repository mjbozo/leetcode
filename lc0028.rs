// LeetCode problem 0028: Find the Indes of the First Occurrence in a String
// https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/description/

fn main() {
    let result = str_str(String::from("sadbutsad"), String::from("sad"));
    println!("Result = {result}");
}

fn str_str(haystack: String, needle: String) -> i32 {
    for (index, window) in haystack.as_bytes().windows(needle.len()).enumerate() {
        if window == needle.as_bytes() {
            return index as i32;
        }
    }

    return -1;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = str_str(String::from("sadbutsad"), String::from("sad"));
        assert_eq!(result, 0);
    }

    #[test]
    fn example_two() {
        let result = str_str(String::from("leetcode"), String::from("leeto"));
        assert_eq!(result, -1);
    }
}
