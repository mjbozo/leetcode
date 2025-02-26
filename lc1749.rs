// LeetCode problem 1749: Maximum Absolute Sum of Any Subarray
// https://leetcode.com/problems/maximum-absolute-sum-of-any-subarray/description/

fn main() {
    let result = max_absolute_sum(vec![1, -3, 2, 3, -4]);
    println!("Result = {result}");
}

fn max_absolute_sum(nums: Vec<i32>) -> i32 {
    let mut most_pos = 0;
    let mut most_neg = 0;

    let mut total = 0;
    for num in nums {
        total += num;
        most_pos = most_pos.max(total);
        most_neg = most_neg.min(total);
    }

    return (most_pos - most_neg).abs();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = max_absolute_sum(vec![1, -3, 2, 3, -4]);
        assert_eq!(result, 5);
    }

    #[test]
    fn example_two() {
        let result = max_absolute_sum(vec![2, -5, 1, -4, 3, -2]);
        assert_eq!(result, 8);
    }
}
