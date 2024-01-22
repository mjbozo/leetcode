// LeetCode problem 0287: Find the Duplicate Number
// https://leetcode.com/problems/find-the-duplicate-number/description/

fn main() {
    let result = find_duplicate(vec![1, 3, 4, 2, 2]);
    println!("Result = {result}");
}

fn find_duplicate(nums: Vec<i32>) -> i32 {
    for i in 1..nums.len() {
        let mut sum: i32 = 0;
        for num in &nums {
            if *num == i as i32 {
                sum += num;
            }
        }

        if sum > 0 && sum != i as i32 {
            return i as i32;
        }

        sum = 0
    }

    return 0;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = find_duplicate(vec![1, 3, 4, 2, 2]);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_two() {
        let result = find_duplicate(vec![3, 1, 3, 4, 2]);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_three() {
        let result = find_duplicate(vec![2, 2, 2, 2, 2]);
        assert_eq!(result, 2);
    }
}

