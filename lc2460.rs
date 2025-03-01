// LeetCode problem 2460: Apply Operations to an Array
// https://leetcode.com/problems/apply-operations-to-an-array/description/

fn main() {
    let result = apply_operations(vec![1, 2, 2, 1, 1, 0]);
    println!("Result = {result:?}");
}

fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let mut res = vec![];
    let mut zero_count = 0;

    for i in 0..nums.len() - 1 {
        if nums[i] == nums[i + 1] {
            if nums[i] == 0 {
                zero_count += 1;
            } else {
                res.push(nums[i] * 2);
                nums[i + 1] = 0;
            }
        } else {
            if nums[i] == 0 {
                zero_count += 1;
            } else {
                res.push(nums[i]);
            }
        }
    }

    if nums[nums.len() - 1] == 0 {
        zero_count += 1;
    } else {
        res.push(nums[nums.len() - 1]);
    }

    res.append(&mut vec![0; zero_count]);
    return res;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = apply_operations(vec![1, 2, 2, 1, 1, 0]);
        assert_eq!(result, vec![1, 4, 2, 0, 0, 0]);
    }

    #[test]
    fn example_two() {
        let result = apply_operations(vec![0, 1]);
        assert_eq!(result, vec![1, 0]);
    }
}
