// LeetCode problem 0007: Reverse Integer
// https://leetcode.com/problems/reverse-integer/description/

fn main() {
    let result = reverse(-123);
    println!("Result = {result}");
}

fn reverse(x: i32) -> i32 {
    let negative = x < 0;

    // converting input to a String, filtering out any characters not a digit (e.g. '-'), reversing, and collecting
    // into a String again to be parsed
    let rev_string = x.to_string().chars().filter(|c| c.is_digit(10)).rev().collect::<String>();

    let mut reversed = match rev_string.parse::<i32>() {
        Ok(n) => n,
        Err(_) => 0
    };

    if negative {
        reversed *= -1;
    }

    return reversed;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = reverse(123);
        assert_eq!(result, 321);
    }

    #[test]
    fn example_two() {
        let result = reverse(-123);
        assert_eq!(result, -321);
    }

    #[test]
    fn example_three() {
        let result = reverse(120);
        assert_eq!(result, 21);
    }
}
