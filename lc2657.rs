// LeetCode problem 2657: Find the Prefix Common Array of Two Arrays
// https://leetcode.com/problems/find-the-prefix-common-array-of-two-arrays/description/

use std::collections::HashSet;

fn main() {
    let result = find_the_prefix_common_array(vec![1, 3, 2, 4], vec![3, 1, 2, 4]);
    println!("Result = {result:?}");
}

fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let n = a.len();
    let mut c = vec![0; n];
    let mut a_seen = HashSet::new();
    let mut b_seen = HashSet::new();

    for i in 0..n {
        let a_val = a[i];
        let b_val = b[i];

        a_seen.insert(a_val);
        b_seen.insert(b_val);

        c[i] = a_seen.intersection(&b_seen).count() as i32;
    }

    return c;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = find_the_prefix_common_array(vec![1, 3, 2, 4], vec![3, 1, 2, 4]);
        assert_eq!(result, vec![0, 2, 3, 4]);
    }

    #[test]
    fn example_two() {
        let result = find_the_prefix_common_array(vec![2, 3, 1], vec![3, 1, 2]);
        assert_eq!(result, vec![0, 1, 3]);
    }
}
