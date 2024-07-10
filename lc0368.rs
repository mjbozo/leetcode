// LeetCode problem 0368: Largest Divisible Subset
// https://leetcode.com/problems/largest-divisible-subset/description/

fn main() {
    let result = largest_divisible_subset(vec![1, 2, 3]);
    println!("Result = {result:?}");
}

fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
    let mut maximum = vec![];
    let mut nums = nums;
    nums.sort();

    for i in 0..nums.len() {
        let mut current = vec![nums[i]];
        for j in (i + 1)..nums.len() {
            if nums[j] % current[current.len() - 1] == 0 {
                current.push(nums[j]);
            }
        }

        if current.len() > maximum.len() {
            maximum = current;
        }
    }

    return maximum;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = largest_divisible_subset(vec![1, 2, 3]);
        assert!(result == vec![1, 2] || result == vec![1, 3]);
    }

    #[test]
    fn example_two() {
        let result = largest_divisible_subset(vec![1, 2, 4, 8]);
        assert_eq!(result, vec![1, 2, 4, 8]);
    }
}
