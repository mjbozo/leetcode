// LeetCode problem 1590: Make Sum Divisible by P
// https://leetcode.com/problems/make-sum-divisible-by-p/description/

use std::collections::HashMap;

fn main() {
    let result = min_subarray(vec![1000000000, 1000000000, 1000000000], 3);
    println!("Result = {result}");
}

fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
    let num_len = nums.len();
    let mut total = 0;

    for n in &nums {
        total = (total + *n) % p;
    }

    let target = total % p;
    if target == 0 {
        return 0;
    }

    let mut mods = HashMap::<i32, i32>::new();
    mods.insert(0, -1);

    let mut current = 0;
    let mut min = num_len as i32;

    for i in 0..num_len {
        current = (current + nums[i]) % p;
        let diff = (current - target + p) % p;

        if let Some(v) = mods.get(&diff) {
            if (i as i32 - *v) < min {
                min = (i as i32 - *v);
            }
        }

        mods.entry(current)
            .and_modify(|v| *v = i as i32)
            .or_insert(i as i32);
    }

    return if min == num_len as i32 { -1 } else { min };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = min_subarray(vec![3, 1, 4, 2], 6);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_two() {
        let result = min_subarray(vec![6, 3, 5, 2], 9);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_three() {
        let result = min_subarray(vec![1, 2, 3], 3);
        assert_eq!(result, 0);
    }
}
