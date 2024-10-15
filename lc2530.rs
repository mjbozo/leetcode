// LeetCode problem 2530: Maximal Score After Applying K Operations
// https://leetcode.com/problems/maximal-score-after-applying-k-operations/description/

use std::collections::BinaryHeap;

fn main() {
    let result = max_kelements(vec![10, 10, 10, 10, 10], 5);
    println!("Result = {result}");
}

fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
    let mut heap = BinaryHeap::with_capacity(nums.len());
    for num in nums {
        heap.push(num);
    }

    let mut total = 0;
    for _ in 0..k {
        let value = heap.pop().unwrap() as i64;
        total += value;
        heap.push((value as f64 / 3.0).ceil() as i32);
    }

    return total;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = max_kelements(vec![10, 10, 10, 10, 10], 5);
        assert_eq!(result, 50);
    }

    #[test]
    fn example_two() {
        let result = max_kelements(vec![1, 10, 3, 3, 3], 3);
        assert_eq!(result, 17);
    }
}
