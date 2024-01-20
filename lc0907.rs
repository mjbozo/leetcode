// LeetCode problem 0907: Sum of Subarray Minimums
// https://leetcode.com/problems/sum-of-subarray-minimums/description/

use std::collections::VecDeque;

fn main() {
    let result = sum_subarray_mins(vec![3, 1, 2, 4]);
    println!("Result = {result}");
}

// solution is correct but is too slow
fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
    let mut sum: usize = 0;
    let mut a = vec![-i32::MAX];
    a.extend(arr.clone());
    a.push(-i32::MAX);
    let mut stack: VecDeque<i32> = VecDeque::with_capacity(arr.len());

    for (index, &num) in a.iter().enumerate() {
        while stack.len() > 0 && a[stack[stack.len() as usize - 1] as usize] > num {
            let j = stack.pop_back().unwrap_or(0);
            let k = stack[stack.len() as usize - 1];
            sum += a[j as usize] as usize * (index as i32 - j) as usize * (j - k) as usize;
        }
        stack.push_back(index as i32);
    }

    return (sum % 1000000007) as i32;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = sum_subarray_mins(vec![3, 1, 2, 4]);
        assert_eq!(result, 17);
    }

    #[test]
    fn example_two() {
        let result = sum_subarray_mins(vec![11, 81, 94, 43, 3]);
        assert_eq!(result, 444);
    }
}
