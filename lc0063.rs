// LeetCode problem 0063: Unique Paths II
// https://leetcode.com/problems/unique-paths-ii/description/

fn main() {
    let result = unique_paths_with_obstacles(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]);
    println!("Result = {result}");
}

fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let m = obstacle_grid.len();
    let n = obstacle_grid[0].len();
    let mut dp = vec![vec![0; n]; m];

    let mut encountered_obstacle = false;
    for i in 0..m {
        encountered_obstacle |= obstacle_grid[i][0] == 1;
        dp[i][0] = if encountered_obstacle { 0 } else { 1 };
    }

    encountered_obstacle = false;
    for i in 0..n {
        encountered_obstacle |= obstacle_grid[0][i] == 1;
        dp[0][i] = if encountered_obstacle { 0 } else { 1 };
    }

    for y in 1..m {
        for x in 1..n {
            dp[y][x] = if obstacle_grid[y][x] == 1 {
                0
            } else {
                dp[y - 1][x] + dp[y][x - 1]
            }
        }
    }

    return *dp.last().unwrap().last().unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = unique_paths_with_obstacles(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_two() {
        let result = unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0]]);
        assert_eq!(result, 1);
    }
}
