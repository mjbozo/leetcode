// LeetCode problem 1481: Least Number of Unique Integers after K Removals
// https://leetcode.com/problems/least-number-of-unique-integers-after-k-removals/description/

use std::collections::HashMap;

fn main() {
    let result = find_least_num_of_unique_ints(vec![5, 5, 4], 0);
    println!("Result = {result}");
}

fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
    let mut counts = HashMap::new();

    for num in &arr {
        counts.entry(num).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut counts = counts.iter().collect::<Vec<_>>();
    counts.sort_unstable_by(|a, b| a.1.cmp(&b.1));

    let mut k = k;
    let len = counts.len();
    for i in 0..len {
        if *counts[i].1 > k {
            return (len - i) as i32;
        } else {
            k -= counts[i].1;
        }
    }

    return 0;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = find_least_num_of_unique_ints(vec![5, 5, 4], 1);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_two() {
        let result = find_least_num_of_unique_ints(vec![4, 3, 1, 1, 3, 3, 2], 3);
        assert_eq!(result, 2);
    }
}
