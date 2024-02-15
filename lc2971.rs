// LeetCode problem 2971: Find Polygon With the Largest Perimeter
// https://leetcode.com/problems/find-polygon-with-the-largest-perimeter/description/

fn main() {
    let result = largest_perimeter(vec![3, 4, 5]);
    println!("Result = {result}");
}

fn largest_perimeter(nums: Vec<i32>) -> i64 {
    let mut nums = nums;
    nums.sort_unstable_by(|a, b| b.cmp(&a));
    let mut total_sum: i64 = nums.iter().map(|&v| v as i64).sum();

    for n in nums.iter() {
        let n = *n as i64;
        total_sum -= n;
        if total_sum > n {
            return total_sum + n;
        }
    }

    return -1;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = largest_perimeter(vec![5, 5, 5]);
        assert_eq!(result, 15);
    }

    #[test]
    fn example_two() {
        let result = largest_perimeter(vec![1, 12, 1, 2, 5, 50, 3]);
        assert_eq!(result, 12);
    }

    #[test]
    fn example_three() {
        let result = largest_perimeter(vec![5, 5, 40]);
        assert_eq!(result, -1);
    }
}
