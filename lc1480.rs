// LeetCode problem 1480: Running Sum of 1d Array
// https://leetcode.com/problems/running-sum-of-1d-array/description/

fn main() {
    let result = running_sum(vec![1, 2, 3, 4]);
    println!("Result = {result:?}");
}

fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut running = Vec::with_capacity(n);
    running.push(nums[0]);
    for i in 1..n {
        running.push(running[i - 1] + nums[i]);
    }
    return running;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = running_sum(vec![1, 2, 3, 4]);
        assert_eq!(result, vec![1, 3, 6, 10]);
    }

    #[test]
    fn example_two() {
        let result = running_sum(vec![1, 1, 1, 1, 1]);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn example_three() {
        let result = running_sum(vec![3, 1, 2, 10, 1]);
        assert_eq!(result, vec![3, 4, 6, 16, 17]);
    }
}
