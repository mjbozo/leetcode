// LeetCode problem 3151: Special Array I
// https://leetcode.com/problems/special-array-i/description/

fn main() {
    let result = is_array_special(vec![1]);
    println!("Result = {result}");
}

fn is_array_special(nums: Vec<i32>) -> bool {
    return nums.windows(2).all(|w| w[0] % 2 + w[1] % 2 == 1);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = is_array_special(vec![1]);
        assert!(result);
    }

    #[test]
    fn example_two() {
        let result = is_array_special(vec![2, 1, 4]);
        assert!(result);
    }

    #[test]
    fn example_three() {
        let result = is_array_special(vec![4, 3, 1, 6]);
        assert!(!result);
    }
}
