// LeetCode problem 1524: Number of Sub-arrays With Odd Sum
// https://leetcode.com/problems/number-of-sub-arrays-with-off-sum/description/

fn main() {
    let result = num_of_subarrays(vec![1, 2, 3, 4, 5, 6, 7]);
    println!("Result = {result}");
}

fn num_of_subarrays(arr: Vec<i32>) -> i32 {
    let mut result: i64 = 0;
    let mut odd_sums = 0;
    let mut even_sums = 0;

    let mut total = 0;
    for num in arr {
        total += num;
        if total % 2 == 1 {
            odd_sums += 1;
            result += (even_sums + 1) as i64;
        } else {
            even_sums += 1;
            result += odd_sums as i64;
        }
    }

    return (result % 1000000007) as i32;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = num_of_subarrays(vec![1, 3, 5]);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_two() {
        let result = num_of_subarrays(vec![2, 4, 6]);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_three() {
        let result = num_of_subarrays(vec![1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(result, 16);
    }
}
