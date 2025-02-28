// LeetCode problem 0873: Length of Longest Fibonacci Subsequence
// https://leetcode.com/problems/length-of-longest-fibonacci-subsequence/description/

use std::collections::HashMap;

fn main() {
    let result = len_longest_fib_subseq(vec![1, 2, 3, 4, 5, 6, 7, 8]);
    println!("Result = {result}");
}

fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut longest = 0;
    let mut dp = vec![vec![0; n]; n];
    let mut val_idxs = HashMap::new();

    for i in 0..n {
        for j in i + 1..n {
            if let Some(&x) = val_idxs.get(&(arr[j] - arr[i])) {
                dp[i][j] = dp[x as usize][i] + 1;
            } else {
                dp[i][j] = 2;
            }

            longest = longest.max(dp[i][j]);
        }
        val_idxs.entry(arr[i]).and_modify(|v| *v = i).or_insert(i);
    }

    return longest;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = len_longest_fib_subseq(vec![1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(result, 5);
    }

    #[test]
    fn example_two() {
        let result = len_longest_fib_subseq(vec![1, 3, 7, 11, 12, 14, 18]);
        assert_eq!(result, 3);
    }
}
