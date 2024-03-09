// LeetCode problem 2540: Minimum Common Value
// https://leetcode.com/problems/minimum-common-value/description/

use std::cmp::Ordering;

fn main() {
    let result = get_common(vec![1,2,3], vec![4,5]);
    println!("Result = {result}");
}

fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = 0;

    while left < nums1.len() && right < nums2.len() {
        match nums1[left].cmp(&nums2[right]) {
            Ordering::Less => left += 1,
            Ordering::Greater => right += 1,
            Ordering::Equal => return nums1[left]
        }
    }

    return -1;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = get_common(vec![1,2,3], vec![2,4]);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_two() {
        let result = get_common(vec![1,2,3,6], vec![2,3,4,5]);
        assert_eq!(result, 2);
    }
}
