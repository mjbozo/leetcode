// LeetCode problem 0268: Missing Number
// https://leetcode.com/problems/missing-number/description/

fn main() {
    let result = missing_number(vec![3, 2, 1]);
    println!("Result = {result}");
}

fn missing_number(nums: Vec<i32>) -> i32 {
    let len = nums.len() as i32;
    let expected_sum = (len * len + len) / 2;
    let actual_sum: i32 = nums.iter().sum();
    return expected_sum - actual_sum;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = missing_number(vec![3, 0, 1]);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_two() {
        let result = missing_number(vec![0, 1]);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_three() {
        let result = missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]);
        assert_eq!(result, 8);
    }
}