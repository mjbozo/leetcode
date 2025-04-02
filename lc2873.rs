// LeetCode problem 2873: Maximum Value of an Ordered Triplet I
// https://leetcode.com/problems/maximum-value-of-an-ordered-triplet-i/description/

fn main() {
    let result = maximum_triplet_value(vec![12, 5, 1, 2, 7]);
    println!("Result = {result}");
}

fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
    let mut maximum = 0;
    let n = nums.len();

    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let triplet = (nums[i] as i64 - nums[j] as i64) * nums[k] as i64;
                maximum = maximum.max(triplet);
            }
        }
    }

    return maximum;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = maximum_triplet_value(vec![12, 5, 1, 2, 7]);
        assert_eq!(result, 77);
    }

    #[test]
    fn example_two() {
        let result = maximum_triplet_value(vec![1, 10, 3, 4, 19]);
        assert_eq!(result, 133);
    }

    #[test]
    fn example_three() {
        let result = maximum_triplet_value(vec![1, 2, 3]);
        assert_eq!(result, 0);
    }
}
