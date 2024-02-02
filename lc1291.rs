// LeetCode problem 1291: Sequential Digits
// https://leetcode.com/problems/sequential-digits/description/

fn main() {
    let result = sequential_digits(100, 300);
    println!("Result = {result:?}");
}

fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
    let possibles = vec![
        12, 23, 34, 45, 56, 67, 78, 89,
        123, 234, 345, 456, 567, 678, 789,
        1234, 2345, 3456, 4567, 5678, 6789,
        12345, 23456, 34567, 45678, 56789,
        123456, 234567, 345678, 456789,
        1234567, 2345678, 3456789,
        12345678, 23456789,
        123456789
    ];

    let mut result = vec![];
    for possibility in possibles {
        if possibility >= low && possibility <= high {
            result.push(possibility);
        }
    }

    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = sequential_digits(100, 300);
        assert_eq!(result, vec![123, 234]);
    }

    #[test]
    fn example_two() {
        let result = sequential_digits(1000, 13000);
        assert_eq!(result, vec![1234, 2345, 3456, 4567, 5678, 6789, 12345]);
    }
}
