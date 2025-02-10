// LeetCode problem 2364: Count Number of Bad Pairs
// https://leetcode.com/problems/count-number-of-bad-pairs/description/

fn main() {
    let result = count_bad_pairs(vec![4, 1, 3, 3]);
    println!("Result = {result}");
}

fn count_bad_pairs(nums: Vec<i32>) -> i64 {
    let mut diffs = std::collections::HashMap::new();
    let n = nums.len();
    for i in 0..n {
        diffs
            .entry(nums[i] - i as i32)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }

    let mut good_pairs = 0;
    for (_, v) in diffs {
        good_pairs += v * (v - 1) / 2;
    }

    return (n * (n - 1) / 2) as i64 - good_pairs;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = count_bad_pairs(vec![4, 1, 3, 3]);
        assert_eq!(result, 5);
    }

    #[test]
    fn example_two() {
        let result = count_bad_pairs(vec![1, 2, 3, 4, 5]);
        assert_eq!(result, 0);
    }
}
