// LeetCode problem 2226: Maximum Candies Allocated to K Children
// https://leetcode.com/problems/maximum-candies-allocated-to-k-children/description/

fn main() {
    let result = maximum_candies(vec![5, 8, 6], 3);
    println!("Result = {result}");
}

fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
    let mut total = 0;
    let mut max = 0;

    for num in candies.iter() {
        total += *num as i64;
        if *num > max {
            max = *num;
        }
    }

    if total < k {
        return 0;
    }

    let mut left = 1;
    let mut right = max;
    let mut result = 0;

    while left <= right {
        let mid = (left + right) / 2;
        if can_divide(&candies, mid, k) {
            result = mid;
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    return result;
}

fn can_divide(arr: &Vec<i32>, size: i32, k: i64) -> bool {
    let mut count = 0;
    for num in arr {
        count += (num / size) as i64;
    }
    return count >= k;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = maximum_candies(vec![5, 8, 6], 3);
        assert_eq!(result, 5);
    }

    #[test]
    fn example_two() {
        let result = maximum_candies(vec![2, 5], 11);
        assert_eq!(result, 0);
    }
}
