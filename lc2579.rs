// LeetCode problem 2579: Count Total Number of Colored Cells
// https://leetcode.com/problems/count-total-number-of-colored-cells/description/

fn main() {
    let result = colored_cells(1);
    println!("Result = {result}");
}

fn colored_cells(n: i32) -> i64 {
    let n = n as i64;
    return (2 * n * n) - (2 * n) + 1;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = colored_cells(1);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_two() {
        let result = colored_cells(2);
        assert_eq!(result, 5);
    }
}
