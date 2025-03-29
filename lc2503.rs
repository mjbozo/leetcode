// LeetCode problem 2503: Maximum Number of Points From Grid Queries
// https://leetcode.com/problems/maximum-number-of-points-from-grid-queries/description/

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

fn main() {
    let result = max_points(
        vec![vec![1, 2, 3], vec![2, 5, 7], vec![3, 5, 1]],
        vec![5, 6, 2],
    );
    println!("Result = {result:?}");
}

fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    let h = grid.len();
    let w = grid[0].len();

    let mut indexed_queries = queries.iter().enumerate().collect::<Vec<_>>();
    indexed_queries.sort_unstable_by_key(|v| v.1);

    let mut points = vec![0; queries.len()];
    let mut processed = HashSet::new();
    let mut current_points = 0;
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((grid[0][0], 0, 0)));
    processed.insert((0, 0));

    for (i, q) in &indexed_queries {
        let v = **q;

        while queue.len() > 0 && queue.peek().unwrap().0 .0 < v {
            let current = queue.pop().unwrap().0;
            let x = current.1;
            let y = current.2;
            current_points += 1;

            let dirs: Vec<(i32, i32)> = vec![(0, -1), (-1, 0), (0, 1), (1, 0)];

            for dir in dirs {
                let nx = x + dir.0;
                let ny = y + dir.1;
                if nx >= 0
                    && nx <= w as i32 - 1
                    && ny >= 0
                    && ny <= h as i32 - 1
                    && !processed.contains(&(nx, ny))
                {
                    queue.push(Reverse((grid[ny as usize][nx as usize], nx, ny)));
                    processed.insert((nx, ny));
                }
            }
        }

        points[*i] = current_points;
    }

    return points;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = max_points(
            vec![vec![1, 2, 3], vec![2, 5, 7], vec![3, 5, 1]],
            vec![5, 6, 2],
        );
        assert_eq!(result, vec![5, 8, 1]);
    }

    #[test]
    fn example_two() {
        let result = max_points(vec![vec![5, 2, 1], vec![1, 1, 2]], vec![3]);
        assert_eq!(result, vec![0]);
    }
}
