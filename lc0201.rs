// LeetCode problem 0201: Bitwise AND of Numbers Range
// https://leetcode.com/problems/bitwise-and-of-numbers-range/description/

fn main() {
    let result = range_bitwise_and(1073741824, 2147483647);
    println!("Result = {result}");
}

fn range_bitwise_and(left: i32, right: i32) -> i32 {
    // diff will represent bits that change between left and right
    let bounds = left & right;
    if bounds == 0 {
        return 0;
    }
    let diff = right - left;
    let num_bits = diff.count_ones() + diff.count_zeros() - diff.leading_zeros();
    let mask = i32::MAX << num_bits;
    return bounds & mask;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = range_bitwise_and(5, 7);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_two() {
        let result = range_bitwise_and(0, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_three() {
        let result = range_bitwise_and(1, 2147483647);
        assert_eq!(result, 0);
    }
}
