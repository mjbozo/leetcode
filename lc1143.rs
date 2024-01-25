// LeetCode problem 1143: Longest Common Subsequence
// https://leetcode.com/problems/longest-common-subsequence/description/

use std::cmp;

fn main() {
    let result = longest_common_subsequence(String::from("abcde"), String::from("ace"));
    println!("Result = {result}");
}

fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let mut dp: Vec<Vec<i32>> = vec![vec![0; text2.len() + 1]; text1.len() + 1];
    for (i, c1) in text1.chars().enumerate() {
        for (j ,c2) in text2.chars().enumerate() {
            if c1 == c2 {
                dp[i + 1][j + 1] = dp[i][j] + 1
            } else {
                dp[i + 1][j + 1] = cmp::max(dp[i][j + 1], dp[i + 1][j]);
            }
        }
    }
    
    return *dp.last().unwrap_or(&vec![]).last().unwrap_or(&0);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = longest_common_subsequence(String::from("abcde"), String::from("ace"));
        assert_eq!(result, 3);
    }

    #[test]
    fn example_two() {
        let result = longest_common_subsequence(String::from("abc"), String::from("abc"));
        assert_eq!(result, 3);
    }

    #[test]
    fn example_three() {
        let result = longest_common_subsequence(String::from("abc"), String::from("def"));
        assert_eq!(result, 0);
    }
}
