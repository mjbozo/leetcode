// LeetCode problem 2419: Longest Subarray With Maximum Bitwise AND
// https://leetcode.com/problems/longest-subarray-with-maximum-bitwise-and/description/

fn main() {
    let result = longest_subarray(vec![1, 2, 3, 3, 2, 2]);
    println!("Result = {result}");
}

fn longest_subarray(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut longest = 0;
    let mut current = 0;

    for &num in nums.iter() {
        if num > max {
            max = num;
            longest = 0;
            current = 1;
            continue;
        }

        if num == max {
            current += 1;
            continue;
        }

        if current > 0 && num < max {
            if current > longest {
                longest = current;
            }
            current = 0;
        }
    }

    if current > longest {
        longest = current;
    }

    return longest;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = longest_subarray(vec![1, 2, 3, 3, 2, 2]);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_two() {
        let result = longest_subarray(vec![1, 2, 3, 4]);
        assert_eq!(result, 1);
    }
}
