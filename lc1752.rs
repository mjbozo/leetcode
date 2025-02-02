// LeetCode problem 1752: Check if Array Is Sorted and Rotated
// https://leetcode.com/problems/check-if-array-is-sorted-and-rotated/description/

fn main() {
    let result = check(vec![3, 4, 5, 1, 2]);
    println!("Result = {result}");
}

fn check(nums: Vec<i32>) -> bool {
    let mut rotated = false;
    for w in nums.windows(2) {
        if w[1] < w[0] {
            if rotated {
                return false;
            }

            rotated = true;
        }
    }

    return if rotated {
        nums.last().unwrap() <= nums.first().unwrap()
    } else {
        true
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = check(vec![3, 4, 5, 1, 2]);
        assert!(result);
    }

    #[test]
    fn example_two() {
        let result = check(vec![2, 1, 3, 4]);
        assert!(!result);
    }

    #[test]
    fn example_three() {
        let result = check(vec![1, 2, 3]);
        assert!(result);
    }
}
