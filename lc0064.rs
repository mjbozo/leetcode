// LeetCode problem 0064: Minimum Path Sum
// https://leetcode.com/problems/minimum-path-sum/description/

fn main() {
    let result = min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]);
    println!("Result = {result}");
}

fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();

    let mut dp = vec![vec![0; m]; n];
    dp[0][0] = grid[0][0];

    for x in 1..m {
        dp[0][x] = grid[0][x] + dp[0][x - 1];
    }

    for y in 1..n {
        dp[y][0] = grid[y][0] + dp[y - 1][0];
    }

    for y in 1..n {
        for x in 1..m {
            dp[y][x] = grid[y][x] + std::cmp::min(dp[y - 1][x], dp[y][x - 1]);
        }
    }

    return *dp.last().unwrap().last().unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]);
        assert_eq!(result, 7);
    }

    #[test]
    fn example_two() {
        let result = min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]);
        assert_eq!(result, 12);
    }
}
