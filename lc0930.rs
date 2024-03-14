// LeetCode problem 0930: Binary Subarrays With Sum
// https://leetcode.com/problems/binary-subarrays-with-sum/description/

fn main() {
    let result = num_subarrays_with_sum(vec![1,0,1,0,1], 2);
    println!("Result = {result}");
}

fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
    let mut left = 0;
    let mut right = 0;
    let mut streak = 0;
    let mut sum = 0;
    let mut result = 0;

    while right < nums.len() {
        sum += nums[right];

        if nums[right] == 1 {
            streak = 0;
        }

        while left <= right && sum >= goal {
            if sum == goal {
                streak += 1;
            }

            sum -= nums[left];
            left += 1;
        }

        result += streak;
        right += 1;
    }

    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = num_subarrays_with_sum(vec![1,0,1,0,1], 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_two() {
        let result = num_subarrays_with_sum(vec![0,0,0,0,0], 0);
        assert_eq!(result, 15);
    }
}
