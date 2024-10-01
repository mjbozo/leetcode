// LeetCode problem 1497: Check If Array Pairs Are Divisible by K
// https://leetcode.com/problems/check-if-array-pairs-are-divisible-by-k/description/

use std::collections::HashMap;

fn main() {
    let result = can_arrange(vec![39, 5, 30, -8, 46, 1, -10, 10, 8, -6, -5, 10], 40);
    println!("Result = {result}");
}

fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
    // can be pretty substantially optimised

    let mut freq = HashMap::new();

    for x in arr {
        freq.entry(((x % k) + k) % k)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }

    for i in 0..k {
        if i == 0 {
            if freq.get(&i).unwrap_or(&0) % 2 != 0 {
                return false;
            }
        } else if freq.get(&i).unwrap_or(&0) != freq.get(&(k - i)).unwrap_or(&0) {
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = can_arrange(vec![1, 2, 3, 4, 5, 10, 6, 7, 8, 9], 5);
        assert_eq!(result, true);
    }

    #[test]
    fn example_two() {
        let result = can_arrange(vec![1, 2, 3, 4, 5, 6], 7);
        assert_eq!(result, true);
    }

    #[test]
    fn example_three() {
        let result = can_arrange(vec![1, 2, 3, 4, 5, 6], 10);
        assert_eq!(result, false);
    }
}
