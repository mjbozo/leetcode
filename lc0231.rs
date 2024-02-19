// LeetCode problem 0231: Power of Two
// https://leetcode.com/problems/power-of-two/description/

fn main() {
    let result = is_power_of_two(420);
    println!("Result = {result}");
}

fn is_power_of_two(n: i32) -> bool {
    if n <= 0 || n % 2 != 0 {
        return n == 1;
    }

    return n.count_ones() == 1;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = is_power_of_two(1);
        assert!(result);
    }

    #[test]
    fn example_two() {
        let result = is_power_of_two(16);
        assert!(result);
    }

    #[test]
    fn example_three() {
        let result = is_power_of_two(3);
        assert!(!result);
    }
}
