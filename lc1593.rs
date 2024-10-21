// LeetCode problem 1593: Split a String Into the Max Number of Unique Substrings
// https://leetcode.com/problems/split-a-string-into-the-max-number-of-unique-substrings/description/

use std::collections::HashSet;

fn main() {
    let result = max_unique_split(String::from("wwwzfvedwfvhsww"));
    println!("Result = {result}");
}

fn max_unique_split(s: String) -> i32 {
    let mut unique_values = HashSet::new();
    if let Some(x) = split_substring(&s, &mut unique_values) {
        println!("{:?}", unique_values);
        return x;
    }

    return 1;
}

fn split_substring<'a>(sub: &'a str, existing: &mut HashSet<&'a str>) -> Option<i32> {
    if sub.len() == 0 {
        return Some(0);
    }

    let mut max = 0;
    for i in 1..=sub.len() {
        if existing.contains(&sub[0..i]) {
            continue;
        }

        existing.insert(&sub[0..i]);
        if let Some(x) = split_substring(&sub[i..], existing) {
            if x + 1 > max {
                max = x + 1;
            }
        }

        existing.remove(&sub[0..i]);
    }

    if max > 0 {
        return Some(max);
    }

    return None;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = max_unique_split(String::from("ababccc"));
        assert_eq!(result, 5);
    }

    #[test]
    fn example_two() {
        let result = max_unique_split(String::from("aba"));
        assert_eq!(result, 2);
    }

    #[test]
    fn example_three() {
        let result = max_unique_split(String::from("aa"));
        assert_eq!(result, 1);
    }
}
