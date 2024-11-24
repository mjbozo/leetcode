// LeetCode problem 1975: Maximum Matrix Sum
// https://leetcode.com/problems/maximum-matrix-sum/description/

fn main() {
    let result = max_matrix_sum(vec![vec![1, -1], vec![-1, 1]]);
    println!("Result = {result}");
}

fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
    let mut num_negatives = 0;
    let mut abs_sum = 0;
    let mut min_abs = i64::MAX;

    for y in 0..matrix.len() {
        for x in 0..matrix[0].len() {
            if matrix[y][x] < 0 {
                num_negatives += 1;
            }

            let a = matrix[y][x].abs() as i64;
            min_abs = std::cmp::min(min_abs, a);
            abs_sum += a;
        }
    }

    if num_negatives % 2 == 0 {
        return abs_sum;
    }

    return abs_sum - (2 * min_abs);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = max_matrix_sum(vec![vec![1, -1], vec![-1, 1]]);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_two() {
        let result = max_matrix_sum(vec![vec![1, 2, 3], vec![-1, -2, -3], vec![1, 2, 3]]);
        assert_eq!(result, 16);
    }
}
