// LeetCode problem 2257: Count Unguarded Cells in the Grid
// https://leetcode.com/problems/count-unguarded-cells-in-the-grid/description/

fn main() {
    let result = count_unguarded(
        4,
        6,
        vec![vec![0, 0], vec![1, 1], vec![2, 3]],
        vec![vec![0, 1], vec![2, 2], vec![1, 4]],
    );
    println!("Result = {result}");
}

// this worked??
fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
    let m = m as usize;
    let n = n as usize;
    let mut grid = vec![vec![0; n]; m];

    // unguarded = 0
    // guarded = 1
    // guard = 2
    // wall = 3

    for g in guards {
        grid[g[0] as usize][g[1] as usize] = 2;
    }

    for w in walls {
        grid[w[0] as usize][w[1] as usize] = 3;
    }

    for y in 0..m {
        let mut guard_found = false;
        let mut guard_visible = false;

        for x in 0..n {
            if grid[y][x] == 2 {
                guard_found = true;
                guard_visible = true;
            }

            if grid[y][x] == 3 {
                guard_visible = false;
            }

            if grid[y][x] == 0 && guard_visible {
                grid[y][x] = 1;
            }
        }

        if guard_found {
            guard_visible = false;
            // need to go the other way too
            for x in (0..n).rev() {
                if grid[y][x] == 2 {
                    guard_visible = true;
                }

                if grid[y][x] == 3 {
                    guard_visible = false;
                }

                if grid[y][x] == 0 && guard_visible {
                    grid[y][x] = 1;
                }
            }
        }
    }

    for x in 0..n {
        let mut guard_found = false;
        let mut guard_visible = false;

        for y in 0..m {
            if grid[y][x] == 2 {
                guard_found = true;
                guard_visible = true;
            }

            if grid[y][x] == 3 {
                guard_visible = false;
            }

            if grid[y][x] == 0 && guard_visible {
                grid[y][x] = 1;
            }
        }

        if guard_found {
            guard_visible = false;
            // need to go the other way too
            for y in (0..m).rev() {
                if grid[y][x] == 2 {
                    guard_visible = true;
                }

                if grid[y][x] == 3 {
                    guard_visible = false;
                }

                if grid[y][x] == 0 && guard_visible {
                    grid[y][x] = 1;
                }
            }
        }
    }

    let mut unguarded = 0;
    for y in 0..m {
        for x in 0..n {
            if grid[y][x] == 0 {
                unguarded += 1;
            }
        }
    }

    return unguarded;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = count_unguarded(
            4,
            6,
            vec![vec![0, 0], vec![1, 1], vec![2, 3]],
            vec![vec![0, 1], vec![2, 2], vec![1, 4]],
        );
        assert_eq!(result, 7);
    }

    #[test]
    fn example_two() {
        let result = count_unguarded(
            3,
            3,
            vec![vec![1, 1]],
            vec![vec![0, 1], vec![1, 0], vec![2, 1], vec![1, 2]],
        );
        assert_eq!(result, 4);
    }
}
