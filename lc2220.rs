// LeetCode problem 2220: Minimum Bit Flips to Convert Number
// https://leetcode.com/problems/minimum-bit-flips-to-convert-number/description/

fn main() {
    let result = min_bit_flips(10, 7);
    println!("Result = {result}");
}

fn min_bit_flips(start: i32, goal: i32) -> i32 {
    return (start ^ goal).count_ones() as i32;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = min_bit_flips(10, 7);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_two() {
        let result = min_bit_flips(3, 4);
        assert_eq!(result, 3);
    }
}
