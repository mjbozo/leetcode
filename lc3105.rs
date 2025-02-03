// LeetCode problem 3105: Longest Strictly Increasing or Strictly Decreasing Subarray
// https://leetcode.com/problems/longest-strictly-increasing-strictly-decreasing-subarray/description/

fn main() {
    let result = longest_monotonic_subarray(vec![1, 4, 3, 3, 2]);
    println!("Result = {result}");
}

fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return 1;
    }

    let mut longest = 0;
    let mut current_inc = 0;
    let mut current_dec = 0;

    for i in 1..nums.len() {
        if nums[i] > nums[i - 1] {
            current_inc += 1;
            current_dec = 0;
        } else if nums[i] < nums[i - 1] {
            current_dec += 1;
            current_inc = 0;
        } else {
            current_inc = 0;
            current_dec = 0;
        }

        longest = longest.max(current_inc).max(current_dec);
    }

    return longest + 1;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = longest_monotonic_subarray(vec![1, 4, 3, 3, 2]);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_two() {
        let result = longest_monotonic_subarray(vec![3, 3, 3, 3]);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_three() {
        let result = longest_monotonic_subarray(vec![3, 2, 1]);
        assert_eq!(result, 3);
    }
}
