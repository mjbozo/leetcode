// LeetCode problem 1277: Count Square Submatrices with All Ones
// https://leetcode.com/problems/count-square-submatrices-with-all-ones/description/

fn main() {
    let result = count_squares(vec![vec![0, 1, 1, 1], vec![1, 1, 1, 1], vec![0, 1, 1, 1]]);
    println!("Result = {result}");
}

fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut dp = vec![vec![0; n]; m];
    for x in 0..n {
        if matrix[0][x] == 1 {
            dp[0][x] = 1;
        }
    }

    for y in 0..m {
        if matrix[y][0] == 1 {
            dp[y][0] = 1;
        }
    }

    for y in 1..m {
        for x in 1..n {
            if matrix[y][x] == 1 {
                dp[y][x] = dp[y][x - 1].min(dp[y - 1][x - 1]).min(dp[y - 1][x]) + 1;
            }
        }
    }

    println!("dp = {:?}", dp);

    let mut squares = 0;
    for y in 0..m {
        for x in 0..n {
            squares += dp[y][x];
        }
    }

    return squares;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = count_squares(vec![vec![0, 1, 1, 1], vec![1, 1, 1, 1], vec![0, 1, 1, 1]]);
        assert_eq!(result, 15);
    }

    #[test]
    fn example_two() {
        let result = count_squares(vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]]);
        assert_eq!(result, 7);
    }
}
