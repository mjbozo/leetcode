// LeetCode problem 1800: Maximum Ascending Subarray Sum
// https://leetcode.com/problems/maximum-ascending-subarray-sum/description/

fn main() {
    let result = max_ascending_sum(vec![10, 20, 30, 5, 10, 50]);
    println!("Result = {result}");
}

fn max_ascending_sum(nums: Vec<i32>) -> i32 {
    let mut res = nums[0];
    let mut current = nums[0];

    for i in 1..nums.len() {
        if nums[i] > nums[i - 1] {
            current += nums[i];
            res = res.max(current);
        } else {
            current = nums[i];
        }
    }

    return res;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = max_ascending_sum(vec![10, 20, 30, 5, 10, 50]);
        assert_eq!(result, 65);
    }

    #[test]
    fn example_two() {
        let result = max_ascending_sum(vec![10, 20, 30, 40, 50]);
        assert_eq!(result, 150);
    }

    #[test]
    fn example_three() {
        let result = max_ascending_sum(vec![12, 17, 15, 13, 10, 11, 12]);
        assert_eq!(result, 33);
    }
}
