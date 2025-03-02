// LeetCode problem 2570: Merge Two 2D Arrays by Summing Values
// https://leetcode.com/problems/merge-two-2d-arrays-by-summing-values/description/

use std::collections::BTreeMap;

fn main() {
    let result = merge_arrays(
        vec![vec![1, 2], vec![2, 3], vec![4, 5]],
        vec![vec![1, 4], vec![3, 2], vec![4, 1]],
    );
    println!("Result = {result:?}");
}

fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut map = BTreeMap::new();
    for num in nums1 {
        map.insert(num[0], num[1]);
    }
    for num in nums2 {
        map.entry(num[0])
            .and_modify(|v| *v += num[1])
            .or_insert(num[1]);
    }

    return map.into_iter().map(|v| vec![v.0, v.1]).collect::<Vec<_>>();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = merge_arrays(
            vec![vec![1, 2], vec![2, 3], vec![4, 5]],
            vec![vec![1, 4], vec![3, 2], vec![4, 1]],
        );
        assert_eq!(result, vec![vec![1, 6], vec![2, 3], vec![3, 2], vec![4, 6]]);
    }

    #[test]
    fn example_two() {
        let result = merge_arrays(
            vec![vec![2, 4], vec![3, 6], vec![5, 5]],
            vec![vec![1, 3], vec![4, 3]],
        );
        assert_eq!(
            result,
            vec![vec![1, 3], vec![2, 4], vec![3, 6], vec![4, 3], vec![5, 5]]
        );
    }
}
