// LeetCode problem 0645: Set Mismatch
// https://leetcode.com/problems/set-mismatch/description/

use std::collections::HashSet;

fn main() {
    let result = find_error_nums(vec![1, 2, 2, 4]);
    println!("Result = {result:?}");
}

fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let length = nums.len();
    let expected_sum: i32 = ((length * (length + 1)) / 2) as i32;
    let mut actual_sum: i32 = 0;

    let mut unique_nums: HashSet<i32> = HashSet::with_capacity(nums.len());
    let mut duplicate: i32 = 0;
    for &num in nums.iter() {
        actual_sum += num;
        if !unique_nums.insert(num) {
            duplicate = num;
        }
    }

    let missing = expected_sum - (actual_sum - duplicate);
    return vec![duplicate, missing];
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = find_error_nums(vec![1, 2, 2, 4]);
        assert_eq!(result, vec![2, 3]);
    }

    #[test]
    fn example_two() {
        let result = find_error_nums(vec![1, 1]);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn example_three() {
        let result = find_error_nums(vec![3, 2, 2]);
        assert_eq!(result, vec![2, 1]);
    }
}
