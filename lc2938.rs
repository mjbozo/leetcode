// LeetCode problem 2938: Separate Black and White Balls
// https://leetcode.com/problems/separate-black-and-white-balls/description/

fn main() {
    let result = minimum_steps("100101011101011011001010101010".into());
    println!("Result = {result}");
}

fn minimum_steps(s: String) -> i64 {
    let mut multiplier = 0;
    let mut total = 0;

    for c in s.bytes() {
        match c {
            b'0' => total += multiplier,
            b'1' => multiplier += 1,
            _ => {}
        };
    }

    return total;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = minimum_steps("101".into());
        assert_eq!(result, 1);
    }

    #[test]
    fn example_two() {
        let result = minimum_steps("100".into());
        assert_eq!(result, 2);
    }

    #[test]
    fn example_three() {
        let result = minimum_steps("0111".into());
        assert_eq!(result, 0);
    }
}
