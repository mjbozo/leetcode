// LeetCode problem 1043: Partition Array for Maximum Sum
// https://leetcode.com/problems/partition-array-for-maximum-sum/description/

use std::cmp;

fn main() {
    let result = max_sum_after_partitioning(vec![1, 15, 7, 9, 2, 5, 10], 3);
    println!("Result = {result}");
}

fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
    let mut dp: Vec<i32> = vec![0; arr.len() + 1];

    for i in 1..=arr.len() as i32 {
        let end = i - 1;
        let start = cmp::max(end - k + 1, 0);
        let mut current_max = 0;
        
        for j in (start..end + 1).rev() {
            current_max = cmp::max(current_max, arr[j as usize]);
            dp[i as usize] = cmp::max(dp[i as usize], dp[j as usize] + current_max * (i - j) as i32);
        }
    }

    return dp[arr.len()];
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = max_sum_after_partitioning(vec![1, 15, 7, 9, 2, 5, 10], 3);
        assert_eq!(result, 84);
    }

    #[test]
    fn example_two() {
        let result = max_sum_after_partitioning(vec![1, 4, 1, 5, 7, 3, 6, 1, 9, 9, 3], 4);
        assert_eq!(result, 83);
    }

    #[test]
    fn example_three() {
        let result = max_sum_after_partitioning(vec![1], 1);
        assert_eq!(result, 1);
    }
}
