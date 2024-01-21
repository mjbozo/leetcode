// LeetCode problem 0198: House Robber
// https://leetcode.com/problems/house-robber/description/

use std::cmp;

fn main() {
    let result = rob(vec![1, 2, 3, 1]);
    println!("Result = {result}");
}

fn rob(nums: Vec<i32>) -> i32 {
    let mut dp: Vec<i32> = vec![0; nums.len()];
    let mut max = 0;
    for (index, &num) in nums.iter().enumerate() {
        let house_2_away = if index >= 2 { dp[index - 2] } else { 0 };
        let house_3_away = if index >= 3 { dp[index - 3] } else { 0 };
        dp[index] = num + cmp::max(house_2_away, house_3_away);

        if dp[index] > max {
            max = dp[index];
        }
    }

    return max;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = rob(vec![1, 2, 3, 1]);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_two() {
        let result = rob(vec![2, 7, 9, 3, 1, ]);
        assert_eq!(result, 12);
    }
}
