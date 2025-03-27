// LeetCode problem 2780: Minimum Index of a Valid Split
// https://leetcode.com/problems/minimum-index-of-a-valid-split/description/

use std::collections::HashMap;

fn main() {
    let result = minimum_index(vec![1, 2, 2, 2]);
    println!("Result = {result}");
}

fn minimum_index(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut right = HashMap::new();
    for n in &nums {
        right.entry(*n).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut left = HashMap::<i32, usize>::new();
    for i in 0..n {
        left.entry(nums[i]).and_modify(|v| *v += 1).or_insert(1);
        right.entry(nums[i]).and_modify(|v| *v -= 1);

        if *left.get(&nums[i]).unwrap() > (i + 1) / 2
            && *right.get(&nums[i]).unwrap() > (n - i - 1) / 2
        {
            return i as i32;
        }
    }

    return -1;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = minimum_index(vec![1, 2, 2, 2]);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_two() {
        let result = minimum_index(vec![2, 1, 3, 1, 1, 1, 7, 1, 2, 1]);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_three() {
        let result = minimum_index(vec![3, 3, 3, 3, 7, 2, 2]);
        assert_eq!(result, -1);
    }
}
