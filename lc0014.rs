// LeetCode problem 0014: Longest Common Prefix
// https://leetcode.com/problems/longest-common-prefix/description/

fn main() {
    let result = longest_common_prefix(vec![String::from("get paid"), String::from("get laid"), String::from("gatorade")]);
    println!("Result = {result}");
}

fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() == 1 {
        return strs[0].clone();
    }

    let mut length = 0;
    let base = &strs[0];
    while length < base.len() {
        for s in &strs[1..] {
            if length >= s.len() {
                return s.clone();
            }
            
            if s[0..=length] != base[0..=length] {
                return base[0..length].to_string();
            }
        }
        length += 1;
    }

    return base.clone();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = longest_common_prefix(vec![String::from("flower"), String::from("flow"), String::from("flight")]);
        assert_eq!(result, String::from("fl"));
    }

    #[test]
    fn example_two() {
        let result = longest_common_prefix(vec![String::from("dog"), String::from("racecar"), String::from("car")]);
        assert_eq!(result, String::new());
    }

    #[test]
    fn example_three() {
        let result = longest_common_prefix(vec![String::from("ab"), String::from("a")]);
        assert_eq!(result, String::from("a"));
    }
}
