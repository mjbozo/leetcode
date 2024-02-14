// LeetCode problem 2149: Rearrange Array Elements by Sign
// https://leetcode.com/problems/rearrange-array-elements-by-sign/description/

fn main() {
    let result = rearrange_array(vec![3, 1, -2, -5, 2, -4]);
    println!("Result = {result:?}");
}

fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let half_len = len / 2;
    let mut positives = Vec::with_capacity(half_len);
    let mut negatives = Vec::with_capacity(half_len);
    for num in nums {
        if num > 0 {
            positives.push(num);
        } else {
            negatives.push(num);
        }
    }

    let mut consolidated = Vec::with_capacity(len);
    for i in 0..half_len {
        consolidated.push(positives[i]);
        consolidated.push(negatives[i]);
    }

    return consolidated;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = rearrange_array(vec![3, 1, -2, -5, 2, -4]);
        assert_eq!(result, vec![3, -2, 1, -5, 2, -4]);
    }

    #[test]
    fn example_two() {
        let result = rearrange_array(vec![-1, 1]);
        assert_eq!(result, vec![1, -1]);
    }

    #[test]
    fn example_three() {
        let result = rearrange_array(vec![-3, -1, 2, 5, -2, 4]);
        assert_eq!(result, vec![2, -3, 5, -1, 4, -2]);
    }
}
