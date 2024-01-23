// LeetCode problem 1239: Maximum Length of a Concatenated String with Unique Characters
// https://leetcode.com/problems/maximum-length-of-a-concatenated-string-with-unique-characters/description/

use std::collections::HashSet;

fn main() {
    let result = max_length(vec![String::from("aa"), String::from("bb")]);
    println!("Result = {result}");
}

fn max_length(arr: Vec<String>) -> i32 {
    let mut record = 0;

    for (index, sub) in arr.iter().enumerate() {
        if !unique_chars(&sub) {
            continue;
        }

        for i in index+1..arr.len() {
            let sub2 = &arr[i];
            let combined = sub.to_owned() + &sub2;

            if !unique_chars(&combined) {
                continue;
            }

            record = combined.len() as i32;

            let mut next_arr: Vec<String> = vec![combined];
            next_arr.extend_from_slice(&arr[i+1..]);

            let next_result = max_length(next_arr);
            if next_result > record {
                record = next_result;
            }
        }
    }

    return record;
}

fn unique_chars(s: &String) -> bool {
    let mut unique = HashSet::with_capacity(s.len());
    for c in s.chars() {
        if !unique.insert(c) {
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
        let result = max_length(vec![String::from("un"), String::from("iq"), String::from("ue")]);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_two() {
        let result = max_length(vec![String::from("cha"), String::from("r"), String::from("act"), String::from("ers")]);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_three() {
        let result = max_length(vec![String::from("abcdefghijklmnopqrstuvwxyz")]);
        assert_eq!(result, 26);
    }
}