// LeetCode problem 2965: Find Missing and Repeated Values
// https://leetcode.com/problems/find-missing-and-repeated-values/description/

fn main() {
    let result = find_missing_and_repeated_values(vec![vec![1, 3], vec![2, 2]]);
    println!("Result = {result:?}");
}

fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let n = grid.len();
    let mut sum = 0;
    let mut sum_squares = 0;

    for i in 0..n {
        for j in 0..n {
            sum += grid[i][j];
            sum_squares += grid[i][j] * grid[i][j];
        }
    }

    let m = (n * n) as i32;
    let expected_sum = (m * (m + 1)) / 2;
    let expected_sum_squares = (m * (m + 1) * (2 * m + 1)) / 6;

    let sum_diff = expected_sum - sum;
    let squ_diff = expected_sum_squares - sum_squares;

    let a = ((squ_diff / sum_diff) - sum_diff) / 2;
    let b = sum_diff + a;
    return vec![a, b];
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = find_missing_and_repeated_values(vec![vec![1, 3], vec![2, 2]]);
        assert_eq!(result, vec![2, 4]);
    }

    #[test]
    fn example_two() {
        let result =
            find_missing_and_repeated_values(vec![vec![9, 1, 7], vec![8, 9, 2], vec![3, 4, 6]]);
        assert_eq!(result, vec![9, 5]);
    }
}
