// LeetCode problem 2044: Count Number of Maximum Bitwise-OR Subsets
// https://leetcode.com/problems/count-number-of-maximum-bitwise-or-subsets/description/

fn main() {
    let result = count_max_or_subsets(vec![3, 1]);
    println!("Result = {result}");
}

fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
    let n = nums.len() as u32;
    let max_bits_value = (2_i32.pow(n)) - 1;

    let mut max = 0;
    for num in &nums {
        max |= *num;
    }

    let mut subsets = 0;
    let mut counter = 1;
    loop {
        let mut mask = counter;
        let mut current = 0;
        for num in &nums {
            if mask & 1 == 1 {
                current |= *num;
            }
            mask = mask >> 1;
        }

        if current == max {
            subsets += 1;
        }

        if counter == max_bits_value {
            break;
        }

        counter += 1;
    }

    return subsets;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = count_max_or_subsets(vec![3, 1]);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_two() {
        let result = count_max_or_subsets(vec![2, 2, 2]);
        assert_eq!(result, 7);
    }

    #[test]
    fn example_three() {
        let result = count_max_or_subsets(vec![3, 2, 1, 5]);
        assert_eq!(result, 6);
    }
}
