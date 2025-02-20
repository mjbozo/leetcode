// LeetCode problem 1980: Find Unique Binary String
// https://leetcode.com/problems/find-unique-binary-string/description/

use std::collections::HashSet;

fn main() {
    let result = find_different_binary_string(vec![String::from("01"), String::from("10")]);
    println!("Result = {result}");
}

fn find_different_binary_string(nums: Vec<String>) -> String {
    let n = nums.len();
    let m = (2_i32).pow(n as u32);
    let mut all_nums = HashSet::with_capacity(m as usize);
    for i in 0..m {
        all_nums.insert(i);
    }

    for num in nums {
        let x = i32::from_str_radix(&num, 2).unwrap_or(0);
        all_nums.remove(&x);
    }

    return format!("{:0n$b}", all_nums.iter().nth(0).unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = find_different_binary_string(vec![String::from("01"), String::from("10")]);
        assert_eq!(result, String::from("11"));
    }

    #[test]
    fn example_two() {
        let result = find_different_binary_string(vec![String::from("00"), String::from("01")]);
        assert_eq!(result, String::from("11"));
    }

    #[test]
    fn example_three() {
        let result = find_different_binary_string(vec![
            String::from("111"),
            String::from("011"),
            String::from("001"),
        ]);
        assert_eq!(result, String::from("101"));
    }
}
