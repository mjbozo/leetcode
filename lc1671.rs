// LeetCode problem 1671: Minimum Number of Removals to Make Mountain Array
// https://leetcode.com/problems/minimum-number-of-removals-to-make-mountain-array/description/

fn main() {
    let result = minimum_mountain_removals(vec![1, 2, 1, 3, 4, 4]);
    println!("Result = {result}");
}

fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
    let left = &nums;
    let right = &nums.iter().rev().collect::<Vec<_>>();

    let mut left_dp = vec![1; left.len()];
    for i in 1..left.len() {
        for j in 0..i {
            if left[j] < left[i] && left_dp[j] + 1 > left_dp[i] {
                left_dp[i] = left_dp[j] + 1;
            }
        }
    }

    let mut right_dp = vec![1; right.len()];
    for i in 1..right.len() {
        for j in 0..i {
            if right[j] < right[i] && right_dp[j] + 1 > right_dp[i] {
                right_dp[i] = right_dp[j] + 1;
            }
        }
    }

    let right_dp = right_dp.iter().rev().collect::<Vec<_>>();

    let mut max_streak = 0;
    for i in 1..nums.len() - 1 {
        if left_dp[i] + right_dp[i] > max_streak
            && nums[i] >= nums[i - 1]
            && nums[i] >= nums[i + 1]
            && right_dp[i] > right_dp[i + 1]
        {
            max_streak = left_dp[i] + right_dp[i];
        }
    }

    return nums.len() as i32 + 1 - max_streak;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = minimum_mountain_removals(vec![1, 3, 1]);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_two() {
        let result = minimum_mountain_removals(vec![2, 1, 1, 5, 6, 2, 3, 1]);
        assert_eq!(result, 3);
    }
}
