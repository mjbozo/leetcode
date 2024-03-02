// LeetCode problem 0977: Squares of a Sorted Array
// https://leetcode.com/problems/squares-of-a-sorted-array/description/

fn main() {
    let result = sorted_squares(vec![-1]);
    println!("Result = {result:?}");
}

fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut right: i32 = 0;
    let len = nums.len() as i32;

    while right < len && nums[right as usize] < 0 {
        right += 1;
    }

    if right == 0 {
        return nums.iter().map(|&v| v * v).collect::<Vec<_>>();
    }

    if right == len {
        return nums.iter().rev().map(|&v| v * v).collect::<Vec<_>>();
    }

    let mut left = right - 1;
    let mut sorted_squares = vec![];
    while left >= 0 || right < len {
        if left < 0 || (right < len && -nums[left as usize] > nums[right as usize]) {
            let v = nums[right as usize];
            sorted_squares.push(v * v);
            right += 1;
        } else {
            let v = nums[left as usize];
            sorted_squares.push(v * v);
            left -= 1;
        }
    }

    return sorted_squares;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = sorted_squares(vec![-4, -1, 0, 3, 10]);
        assert_eq!(result, vec![0, 1, 9, 16, 100]);
    }

    #[test]
    fn example_two() {
        let result = sorted_squares(vec![-7, -3, 2, 3, 11]);
        assert_eq!(result, vec![4, 9, 9, 49, 121]);
    }
}