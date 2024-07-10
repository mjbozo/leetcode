// LeetCode problem 0001: Two Sum
// https://leetcode.com/problems/two-sum/description/

use std::collections::HashMap;

fn main() {
    let result = two_sum(vec![1, 2, 3, 4, 5], 9);
    println!("Result = {result:?}");
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen: HashMap<i32, i32> = HashMap::new();
    for (index, num) in nums.iter().enumerate() {
        let alt = target - num;
        if seen.contains_key(&alt) {
            let alt_index = seen.get(&alt).expect("Value for key {alt} should exist");
            return vec![index as i32, *alt_index];
        } else {
            seen.insert(*num, index as i32);
        }
    }
    return vec![];
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = two_sum(vec![2, 7, 11, 15], 9);
        assert!(result.contains(&0));
        assert!(result.contains(&1));
    }

    #[test]
    fn example_two() {
        let result = two_sum(vec![3, 2, 4], 6);
        assert!(result.contains(&1));
        assert!(result.contains(&2));
    }

    #[test]
    fn example_three() {
        let result = two_sum(vec![3, 3], 6);
        assert!(result.contains(&0));
        assert!(result.contains(&1));
    }
}