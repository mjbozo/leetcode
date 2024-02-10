// LeetCode problem 0016: 3Sum Closest
// https://leetcode.com/problems/3sum-closest/description/

use std::cmp::Ordering;

fn main() {
    let result = three_sum_closest(vec![-4, 2, 1, -1], 1);
    println!("Result = {result}");
}

fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut closest = nums[0] + nums[1] + nums[2];
    let mut nums = nums;
    nums.sort();

    for i in 0..(nums.len() - 2) {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let mut left = i + 1;
        let mut right = nums.len() - 1;

        while left < right {
            let current = nums[i] + nums[left] + nums[right];
            let diff = target - current;
            if diff.abs() < (target - closest).abs() {
                closest = current;
            }

            match (current).cmp(&target) {
                Ordering::Less => {
                    left += 1;
                }
                Ordering::Greater => {
                    right -= 1;
                }
                Ordering::Equal => {
                    return target;
                }
            }
        }
    }

    return closest;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = three_sum_closest(vec![-1, 2, 1, -4], 1);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_two() {
        let result = three_sum_closest(vec![0, 0, 0], 1);
        assert_eq!(result, 0);
    }
}
