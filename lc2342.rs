// LeetCode problem 2342: Max Sum of a Pair With Equal Sum of Digits
// https://leetcode.com/problems/max-sum-of-a-pair-with-equal-sum-of-digits/description/

use std::collections::HashMap;

fn main() {
    let result = maximum_sum(vec![
        368, 369, 307, 304, 384, 138, 90, 279, 35, 396, 114, 328, 251, 364, 300, 191, 438, 467, 183,
    ]);
    println!("Result = {result}");
}

fn maximum_sum(nums: Vec<i32>) -> i32 {
    let mut maximum = -1;
    let mut digit_sums = HashMap::new();
    for num in nums {
        let mut n = num;
        let mut digit_sum = 0;
        while n > 0 {
            digit_sum += n % 10;
            n /= 10;
        }

        digit_sums
            .entry(digit_sum)
            .and_modify(|v| {
                if *v + num > maximum {
                    maximum = *v + num;
                }

                if num > *v {
                    *v = num;
                }
            })
            .or_insert(num);
    }

    return maximum;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = maximum_sum(vec![18, 43, 36, 13, 7]);
        assert_eq!(result, 54);
    }

    #[test]
    fn example_two() {
        let result = maximum_sum(vec![10, 12, 19, 14]);
        assert_eq!(result, -1);
    }
}
