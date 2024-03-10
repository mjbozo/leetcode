// LeetCode problem 0349: Intersection of Two Arrays
// https://leetcode.com/problems/intersection-of-two-arrays/description/

use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let result = intersection(vec![1,2,2,1], vec![2,2]);
    println!("Result = {result:?}");
}

fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let set1: HashSet<i32> = HashSet::from_iter(nums1.iter().cloned());
    let set2: HashSet<i32> = HashSet::from_iter(nums2.iter().cloned());
    let intersection = set1.intersection(&set2);
    return intersection.into_iter().cloned().collect::<Vec<_>>();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = intersection(vec![1,2,2,1], vec![2,2]);
        assert_eq!(result, vec![2]);
    }

    #[test]
    fn example_two() {
        let result = intersection(vec![4,9,5], vec![9,4,9,8,4]);
        assert_eq!(result, vec![4,9]);
    }
}
