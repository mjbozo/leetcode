// LeetCode problem 0169: Majority Element
// https://leetcode.com/problems/majority-element/description/

use std::collections::HashMap;

fn main() {
    let result = majority_element(vec![3, 2, 3]);
    println!("Result = {result}");
}

fn majority_element(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }
    let mut counts: HashMap<i32, i32> = HashMap::new();
    let half = nums.len() as i32 / 2;
    for n in nums {
        if let Some(x) = counts.get_mut(&n) {
            *x += 1;
            if *x > half {
                return n;
            }
        } else {
            counts.insert(n, 1);
        }
    }

    return 0;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = majority_element(vec![3, 2, 3]);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_two() {
        let result = majority_element(vec![2, 2, 1, 1, 1, 2, 2]);
        assert_eq!(result, 2);
    }
}
