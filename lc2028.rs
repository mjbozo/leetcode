// LeetCode problem 2028: Find Missing Observations
// https://leetcode.com/problems/find-missing-observations/description/

fn main() {
    let result = missing_rolls(vec![3, 2, 4, 3], 4, 2);
    println!("Result = {result:?}");
}

fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
    let rolls_sum: i32 = rolls.iter().sum();
    let rolls_len = rolls.len() as i32;
    let sum_of_missing = (mean * (rolls_len + n)) - rolls_sum;

    if sum_of_missing < n || (sum_of_missing / n) > 6 {
        return vec![];
    }

    let mut result = Vec::with_capacity(n as usize);
    let mut used = 0;
    for i in 0..n {
        let value = (sum_of_missing - used) / (n - i as i32);
        result.push(value);
        used += value;
    }

    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = missing_rolls(vec![3, 2, 4, 3], 4, 2);
        assert_eq!(result, vec![6, 6]);
    }

    #[test]
    fn example_two() {
        let result = missing_rolls(vec![1, 5, 6], 3, 4);
        assert_eq!(result, vec![2, 2, 2, 3]);
    }

    #[test]
    fn example_three() {
        let result = missing_rolls(vec![1, 2, 3, 4], 6, 4);
        assert_eq!(result, vec![]);
    }
}
