// LeetCode problem 3254: Find the Power of K-Size Subarrays I
// https://leetcode.com/problems/find-the-power-of-k-size-subarrays-i/description/

fn main() {
    let result = results_array(vec![1, 2, 3, 4, 3, 2, 5], 3);
    println!("Result = {result:?}");
}

fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = nums.len();
    let s = n - k + 1;
    let mut results = Vec::with_capacity(s);

    let mut left = 0;
    let mut right = k - 1;
    let mut sum = 0;

    for i in 0..k {
        sum += nums[i];
    }

    for i in 0..s {
        let tri = (nums[i + k - 1] * (nums[i + k - 1] + 1)) / 2;
        let sub = ((nums[i] - 1) * nums[i]) / 2;

        if nums[i + k - 1] == nums[i] + k as i32 - 1 && sum == (tri - sub) {
            results.push(nums[i + k - 1]);
        } else {
            results.push(-1);
        }

        if right < n - 1 {
            right += 1;
            sum += nums[right];
            sum -= nums[left];
            left += 1;
        }
    }

    return results;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = results_array(vec![1, 2, 3, 4, 3, 2, 5], 3);
        assert_eq!(result, vec![3, 4, -1, -1, -1]);
    }

    #[test]
    fn example_two() {
        let result = results_array(vec![2, 2, 2, 2, 2], 4);
        assert_eq!(result, vec![-1, -1]);
    }

    #[test]
    fn example_three() {
        let result = results_array(vec![3, 2, 3, 2, 3, 2], 2);
        assert_eq!(result, vec![-1, 3, -1, 3, -1]);
    }
}
