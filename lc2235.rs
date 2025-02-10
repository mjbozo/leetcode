// LeetCode problem 2236: Add Two Integers
// https://leetcode.com/problems/add-two-integers/description/

fn main() {
    let result = sum(12, 5);
    println!("Result = {result}");
}

fn sum(num1: i32, num2: i32) -> i32 {
    return num1 + num2;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = sum(12, 5);
        assert_eq!(result, 17);
    }

    #[test]
    fn example_two() {
        let result = sum(-10, 4);
        assert_eq!(result, -6);
    }
}
