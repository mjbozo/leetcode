// LeetCode problem 3066: Minimum Operations to Exceed Thrshold Value II
// https://leetcode.com/problems/minimum-operations-to-exceed-threshold-value-ii/description/

use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let result = min_operations(vec![2, 11, 10, 1, 3], 10);
    println!("Result = {result}");
}

fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut heap = BinaryHeap::new();
    for num in nums {
        heap.push(Reverse(num as u64));
    }

    let mut i = 0;
    while heap.len() >= 2 && heap.peek().unwrap().0 < k as u64 {
        i += 1;
        let min = heap.pop().unwrap();
        let max = heap.pop().unwrap();
        let new_val = 2 * min.0 + max.0;
        heap.push(Reverse(new_val as u64));
    }

    return i;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = min_operations(vec![2, 11, 10, 1, 3], 10);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_two() {
        let result = min_operations(vec![1, 1, 2, 4, 9], 20);
        assert_eq!(result, 4);
    }
}
