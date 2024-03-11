// LeetCode problem 0791: Custom Sort String
// https://leetcode.com/problems/custom-sort-string/description/

fn main() {
    let result = custom_sort_string(String::from("cba"), String::from("abcd"));
    println!("Result = {result}");
}

fn custom_sort_string(order: String, s: String) -> String {
    let mut lookup = [26; 26];
    for (index, byte) in order.as_bytes().iter().enumerate() {
        lookup[(byte - b'a') as usize] = index;
    }

    let mut bytes = s.into_bytes();
    bytes.sort_unstable_by(|a, b| lookup[(a - b'a') as usize].cmp(&lookup[(b - b'a') as usize]));
    let ordered_string = std::str::from_utf8(&bytes).unwrap().to_string();
    return ordered_string;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = custom_sort_string(String::from("cba"), String::from("abcd"));
        assert_eq!(result, String::from("cbad"));
    }

    #[test]
    fn example_two() {
        let result = custom_sort_string(String::from("bcafg"), String::from("abcd"));
        assert_eq!(result, String::from("bcad"));
    }
}
