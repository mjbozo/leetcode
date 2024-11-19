// LeetCode problem 2461: Maximum Sum of Distinct Subarrays With Length K
// https://leetcode.com/problems/maximum-sum-of-distinct-subarrays-with-length-k/description/

use std::collections::HashMap;

fn main() {
    let result = maximum_subarray_sum(vec![9, 9, 9, 1, 2, 3], 3);
    println!("Result = {result}");
}

fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    let mut max_sum: i64 = 0;
    let mut left = 0;
    let mut right = k as usize - 1;

    let mut current_sum: i64 = 0;
    let mut window = HashMap::<i32, i32>::new();

    for i in left..=right {
        current_sum += nums[i] as i64;
        window.entry(nums[i]).and_modify(|v| *v += 1).or_insert(1);
    }

    while right < nums.len() {
        if window.len() == k as usize {
            // all values are unique
            max_sum = std::cmp::max(max_sum, current_sum);
        }

        // move window along
        window.entry(nums[left]).and_modify(|v| {
            *v -= 1;
        });
        if *window.get(&nums[left]).unwrap_or(&0) == 0 {
            window.remove(&nums[left]);
        }

        current_sum -= nums[left] as i64;
        left += 1;
        right += 1;

        if right < nums.len() {
            current_sum += nums[right] as i64;
            window
                .entry(nums[right])
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
    }

    if window.len() == k as usize {
        // all values are unique
        max_sum = std::cmp::max(max_sum, current_sum);
    }

    return max_sum;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = maximum_subarray_sum(vec![1, 5, 4, 2, 9, 9, 9], 3);
        assert_eq!(result, 15);
    }

    #[test]
    fn example_two() {
        let result = maximum_subarray_sum(vec![4, 4, 4], 3);
        assert_eq!(result, 0);
    }
}
