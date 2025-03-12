// LeetCode problem 2529: Maximum Count of Positive Integer and Negative Integer
// https://leetcode.com/problems/maximum-count-of-positive-integer-and-negative-integer/description/

fn main() {
    let result = maximum_count(vec![-2, -1, -1, 1, 2, 3]);
    println!("Result = {result}");
}

fn maximum_count(nums: Vec<i32>) -> i32 {
    let pos = nums.len() as i32 - binary_search(&nums, 1);
    let neg = binary_search(&nums, 0);
    return pos.max(neg);
}

fn binary_search(nums: &Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() as i32 - 1;
    let mut idx = nums.len() as i32;

    while left <= right {
        let mid = (left + right) / 2;
        if nums[mid as usize] < target {
            left = mid + 1;
        } else {
            idx = mid;
            right = mid - 1;
        }
    }

    return idx as i32;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = maximum_count(vec![-2, -1, -1, 1, 2, 3]);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_two() {
        let result = maximum_count(vec![-3, -2, -1, 0, 0, 1, 2]);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_three() {
        let result = maximum_count(vec![5, 20, 66, 1314]);
        assert_eq!(result, 4);
    }
}
