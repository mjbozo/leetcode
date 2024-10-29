// LeetCode problem 2684: Maximum Number of Moves in a Grid
// https://leetcode.com/problems/maximum-number-of-moves-in-a-grid/description/

fn main() {
    let result = max_moves(vec![
        vec![2, 4, 3, 5],
        vec![5, 4, 9, 3],
        vec![3, 4, 2, 11],
        vec![10, 9, 13, 15],
    ]);
    println!("Result = {result}");
}

fn max_moves(mut grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();

    for c in 1..n {
        let mut path_exists = false;
        for r in 0..m {
            if r > 0 && grid[r - 1][c - 1] < grid[r][c] && grid[r - 1][c - 1] > 0 {
                path_exists = true;
                continue;
            }

            if grid[r][c - 1] < grid[r][c] && grid[r][c - 1] > 0 {
                path_exists = true;
                continue;
            }

            if r < m - 1 && grid[r + 1][c - 1] < grid[r][c] && grid[r + 1][c - 1] > 0 {
                path_exists = true;
                continue;
            }

            grid[r][c] = 0;
        }

        if !path_exists {
            return c as i32 - 1;
        }
    }

    return n as i32 - 1;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = max_moves(vec![
            vec![2, 4, 3, 5],
            vec![5, 4, 9, 3],
            vec![3, 4, 2, 11],
            vec![10, 9, 13, 15],
        ]);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_two() {
        let result = max_moves(vec![vec![3, 2, 4], vec![2, 1, 9], vec![1, 1, 7]]);
        assert_eq!(result, 0);
    }
}
