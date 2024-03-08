// LeetCode problem 3005: Count Elements With Maximum Frequency
// https://leetcode.com/problems/count-elements-with-maximum-frequency/description/

use std::collections::HashMap;

fn main() {
    let result = max_frequency_elements(vec![1,2,2,3,1,4]);
    println!("Result = {result}");
}

fn max_frequency_elements(nums: Vec<i32>) -> i32 {
    let mut frequencies = HashMap::new();
    for num in nums.iter() {
        frequencies.entry(num).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut max_freq = 0;
    let mut total = 0;
    for (_, &value) in frequencies.iter() {
        if value > max_freq {
            max_freq = value;
            total = max_freq;
        } else if value == max_freq {
            total += max_freq;
        }
    }

    return total;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = max_frequency_elements(vec![1,2,2,3,1,4]);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_two() {
        let result = max_frequency_elements(vec![1,2,3,4,5]);
        assert_eq!(result, 5);
    }
}
