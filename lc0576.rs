// LeetCode problem 0576: Out of Boundary Paths
// https://leetcode.com/problems/out-of-boundary-paths/description/

fn main() {
    let result = find_paths(2, 2, 2, 0, 0);
    println!("Result = {result}");
}

fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = find_paths(2, 2, 2, 0, 0);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_two() {
        let result = find_paths(1, 3, 3, 0, 1);
        assert_eq!(result, 12);
    }
}
