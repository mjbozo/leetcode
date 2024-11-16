// LeetCode problem 2563: Count the Number of Fair Pairs
// https://leetcode.com/problems/count-the-number-of-fair-pairs/description/

fn main() {
    let result = count_fair_pairs(vec![0, 1, 7, 4, 4, 5], 3, 6);
    println!("Result = {result}");
}

fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
    let mut fair_pairs: i64 = 0;
    let mut nums = nums;
    nums.sort_unstable();

    let mut left = 0;
    let mut right = nums.len() - 1;

    while left < right {
        let sum = nums[left] + nums[right];
        if sum <= upper {
            fair_pairs += (right - left) as i64;
            left += 1;
        } else {
            right -= 1;
        }
    }

    left = 0;
    right = nums.len() - 1;

    while left < right {
        let sum = nums[left] + nums[right];
        if sum < lower {
            fair_pairs -= (right - left) as i64;
            left += 1;
        } else {
            right -= 1;
        }
    }

    return fair_pairs;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = count_fair_pairs(vec![0, 1, 7, 4, 4, 5], 3, 6);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_two() {
        let result = count_fair_pairs(vec![1, 7, 9, 2, 5], 11, 11);
        assert_eq!(result, 1);
    }
}
