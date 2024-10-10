// LeetCode problem 0962: Maximum Width Ramp
// https://leetcode.com/problems/maximum-width-ramp/description/

fn main() {
    let result = max_width_ramp(vec![6, 0, 8, 2, 1, 5]);
    println!("Result = {result}");
}

fn max_width_ramp(nums: Vec<i32>) -> i32 {
    let mut stack = vec![];

    for (i, n) in nums.iter().enumerate() {
        if stack.is_empty() || nums[*stack.last().unwrap() as usize] > *n {
            stack.push(i);
        }
    }

    let mut max_width = 0;
    for x in (0..nums.len()).rev() {
        while !stack.is_empty() && nums[x] >= nums[*stack.last().unwrap() as usize] {
            let i = stack.pop().unwrap();
            if x - i > max_width {
                max_width = x - i;
            }
        }
    }

    return max_width as i32;
}

fn max_width_ramp_slow(nums: Vec<i32>) -> i32 {
    let mut max_width = 0;

    for i in 0..nums.len() - 1 {
        for j in i + 1..nums.len() {
            if nums[i] <= nums[j] && j - i > max_width {
                max_width = j - i;
            }
        }
    }

    return max_width as i32;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = max_width_ramp(vec![6, 0, 8, 2, 1, 5]);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_two() {
        let result = max_width_ramp(vec![9, 8, 1, 0, 1, 9, 4, 0, 4, 1]);
        assert_eq!(result, 7);
    }
}
