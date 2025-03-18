// LeetCode problem 2401: Longest Nice Subarray
// https://leetcode.com/problems/longest-nice-subarray/description/

fn main() {
    let result = longest_nice_subarray(vec![1, 3, 8, 48, 10]);
    println!("Result = {result}");
}

fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = 0;
    let mut longest = 0;
    let mut bits = vec![0; 32];

    while right < nums.len() {
        let mut current = nums[right];
        for i in 0..32 {
            bits[i] += current & 1;
            current = current >> 1;
        }

        while !bits.iter().all(|v| *v <= 1) {
            let mut remove = nums[left];
            for i in 0..32 {
                bits[i] -= remove & 1;
                remove = remove >> 1;
            }
            left += 1;
        }

        longest = longest.max(right - left + 1);
        right += 1;
    }

    return longest as i32;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = longest_nice_subarray(vec![1, 3, 8, 48, 10]);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_two() {
        let result = longest_nice_subarray(vec![3, 1, 5, 11, 13]);
        assert_eq!(result, 1);
    }
}
