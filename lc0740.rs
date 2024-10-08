// LeetCode problem 0740: Delete and Earn
// https://leetcode.com/problems/delete-and-earn/description/

use std::collections::BTreeMap;

fn main() {
    let result = delete_and_earn(vec![1, 1, 1, 2, 4, 5, 5, 5, 6]);
    println!("Result = {result}");
}

fn delete_and_earn(nums: Vec<i32>) -> i32 {
    let mut freq = BTreeMap::new();
    for x in nums.into_iter() {
        freq.entry(x).and_modify(|v| *v += x).or_insert(x);
    }

    let mut freq_iter = freq.iter();

    let mut dp = vec![0; freq.len()];
    let mut prev_key;
    let first = freq_iter.next().unwrap();
    prev_key = *first.0;
    dp[0] = *first.1;

    if let Some(x) = freq_iter.next() {
        if prev_key == *x.0 - 1 {
            dp[1] = std::cmp::max(dp[0], *x.1);
        } else {
            dp[1] = dp[0] + *x.1;
        }
        prev_key = *x.0;
    }

    for i in 2..freq.keys().len() {
        let current = freq_iter.next().unwrap();
        if prev_key == *current.0 - 1 {
            dp[i] = std::cmp::max(dp[i - 2] + *current.1, dp[i - 1]);
        } else {
            dp[i] = std::cmp::max(dp[i - 2] + *current.1, dp[i - 1] + *current.1);
        }
        prev_key = *current.0;
    }

    return *dp.last().unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = delete_and_earn(vec![3, 4, 2]);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_two() {
        let result = delete_and_earn(vec![2, 2, 3, 3, 3, 4]);
        assert_eq!(result, 9);
    }
}
