// LeetCode problem 2658: Maximum Number of Fish in a Grid
// https://leetcode.com/problems/maximum-number-of-fish-in-a-grid/description/

// A better solution is to update the grid cells value to 0 when its seen to avoid using the `seen` hashset, and using DFS instead of BFS

use std::collections::{HashSet, VecDeque};

fn main() {
    let result = find_max_fish(vec![
        vec![0, 2, 1, 0],
        vec![4, 0, 0, 3],
        vec![1, 0, 0, 4],
        vec![0, 3, 2, 0],
    ]);
    println!("Result = {result}");
}

fn find_max_fish(grid: Vec<Vec<i32>>) -> i32 {
    let mut res = 0;
    let mut seen = HashSet::new();

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] > 0 && !seen.contains(&(r as i32, c as i32)) {
                let pond_size = count_pond(&grid, r, c, &mut seen);
                if pond_size > res {
                    res = pond_size;
                }
            }
        }
    }

    return res;
}

fn count_pond(grid: &Vec<Vec<i32>>, r: usize, c: usize, seen: &mut HashSet<(i32, i32)>) -> i32 {
    let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut queue = VecDeque::new();
    queue.push_back((r as i32, c as i32));

    let mut total = 0;
    while queue.len() > 0 {
        let current = queue.pop_front().unwrap();
        seen.insert(current);
        total += grid[current.0 as usize][current.1 as usize];

        for i in 0..dirs.len() {
            let d = dirs[i];
            let next_r = current.0 + d.0;
            let next_c = current.1 + d.1;

            if next_r < 0
                || next_r >= grid.len() as i32
                || next_c < 0
                || next_c >= grid[0].len() as i32
            {
                continue;
            }

            if grid[next_r as usize][next_c as usize] > 0
                && !seen.contains(&(next_r, next_c))
                && !queue.contains(&(next_r, next_c))
            {
                queue.push_back((next_r, next_c));
            }
        }
    }

    return total;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = find_max_fish(vec![
            vec![0, 2, 1, 0],
            vec![4, 0, 0, 3],
            vec![1, 0, 0, 4],
            vec![0, 3, 2, 0],
        ]);
        assert_eq!(result, 7);
    }

    #[test]
    fn example_two() {
        let result = find_max_fish(vec![
            vec![1, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 1],
        ]);
        assert_eq!(result, 1);
    }
}
