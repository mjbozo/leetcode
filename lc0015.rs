// LeetCode problem 0015: 3Sum
// https://leetcode.com/problems/3sum/description/

use std::cmp::Ordering;

fn main() {
    let result = three_sum(vec![-1, 0, 1, 2, -1, -4]);
    println!("Result = {result:?}");
}

fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut sums: Vec<Vec<i32>> = vec![];
    let mut nums = nums;
    nums.sort();

    for i in 0..(nums.len() - 2) {
        if nums[i] > 0 {
            break;
        }

        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let mut left = i + 1;
        let mut right = nums.len() - 1;

        while left < right {
            match (nums[i] + nums[left] + nums[right]).cmp(&0) {
                Ordering::Less => {
                    left += 1;
                }
                Ordering::Greater => {
                    right -= 1;
                }
                Ordering::Equal => {
                    sums.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;
                    right -= 1;

                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }

                    while left < right && nums[right] == nums[right + 1] {
                        right -= 1;
                    }
                }
            }
        }
    }

    return sums;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = three_sum(vec![-1, 0, 1, 2, -1, -4]);
        assert_eq!(result, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    }

    #[test]
    fn example_two() {
        let result = three_sum(vec![0, 1, 1]);
        assert_eq!(result, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn example_three() {
        let result = three_sum(vec![0, 0, 0]);
        assert_eq!(result, vec![vec![0, 0, 0]]);
    }
}
