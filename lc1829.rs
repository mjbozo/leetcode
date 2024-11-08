// LeetCode problem 1829: Maximum XOR For Each Query
// https://leetcode.com/problems/maximum-xor-for-each-query/description/

fn main() {
    let result = get_maximum_xor(vec![0, 1, 1, 3], 2);
    println!("Result = {result:?}");
}

fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
    let mut nums = nums;
    let mut answers = Vec::with_capacity(nums.len());

    let mut xor_all = 0;
    for num in &nums {
        xor_all ^= *num;
    }

    let max = 2_i32.pow(maximum_bit as u32) - 1;
    while nums.len() > 0 {
        answers.push(xor_all ^ max);
        let last = nums.pop().unwrap();
        xor_all ^= last;
    }

    return answers;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = get_maximum_xor(vec![0, 1, 1, 3], 2);
        assert_eq!(result, vec![0, 3, 2, 3]);
    }

    #[test]
    fn example_two() {
        let result = get_maximum_xor(vec![2, 3, 4, 7], 3);
        assert_eq!(result, vec![5, 2, 6, 5]);
    }

    #[test]
    fn example_three() {
        let result = get_maximum_xor(vec![0, 1, 2, 2, 5, 7], 3);
        assert_eq!(result, vec![4, 3, 6, 4, 6, 7]);
    }
}
