// LeetCode problem 3097: Shortest Subarray With OR at Least K II
// https://leetcode.com/problems/shortest-subarray-with-or-at-least-k-ii/description/

fn main() {
    let result = minimum_subarray_length(vec![1, 2, 3], 2);
    println!("Result = {result}");
}

fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    let mut left = 0;
    let mut right = 0;
    let mut min_length = i32::MAX;
    let mut bit_counts = vec![0; 32];

    while right < nums.len() {
        for i in 0..32 {
            if nums[right] & (1 << i) >= 1 {
                bit_counts[i] += 1;
            }
        }

        let mut current = 0;
        for i in 0..32 {
            if bit_counts[i] >= 1 {
                current |= 1 << i;
            }
        }

        while left <= right && current >= k {
            min_length = std::cmp::min(min_length, right as i32 - left as i32 + 1);
            for i in 0..32 {
                if nums[left] & (1 << i) >= 1 {
                    bit_counts[i] -= 1;
                }
                if bit_counts[i] == 0 {
                    current &= i32::MAX ^ (1 << i);
                }
            }

            left += 1;
        }

        right += 1;
    }

    if min_length == i32::MAX {
        return -1;
    }

    return min_length;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = minimum_subarray_length(vec![1, 2, 3], 2);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_two() {
        let result = minimum_subarray_length(vec![2, 1, 8], 10);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_three() {
        let result = minimum_subarray_length(vec![1, 2], 0);
        assert_eq!(result, 1);
    }
}
