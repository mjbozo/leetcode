// LeetCode problem 2501: Longest Square Streak in an Array
// https://leetcode.com/problems/longest-square-streak-in-an-array/description/

fn main() {
    let result = longest_square_streak(vec![4, 3, 6, 16, 8, 2]);
    println!("Result = {result}");
}

fn longest_square_streak(nums: Vec<i32>) -> i32 {
    let mut longest = -1;
    let mut history = std::collections::HashMap::<i32, i32>::with_capacity(nums.len());

    let mut nums = nums;
    nums.sort_unstable_by(|a, b| b.cmp(&a));

    for num in nums.into_iter() {
        if let Some(&x) = history.get(&(num * num)) {
            history.insert(num, x + 1);
            if x >= 1 && x + 1 > longest {
                longest = x + 1;
            }
        } else {
            history.insert(num, 1);
        }
    }

    return longest;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = longest_square_streak(vec![4, 3, 6, 16, 8, 2]);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_two() {
        let result = longest_square_streak(vec![2, 3, 5, 6, 7]);
        assert_eq!(result, -1);
    }
}
