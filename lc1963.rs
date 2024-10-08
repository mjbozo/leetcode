// LeetCode problem 1964: Minimum Number of Swaps to Make the String Balanced
// https://leetcode.com/problems/minimum-number-of-swaps-to-make-the-string-balanced/description/

fn main() {
    let result = min_swaps("][][".into());
    println!("Result = {result}");
}

fn min_swaps(s: String) -> i32 {
    let mut stack = Vec::with_capacity(s.len());

    for c in s.chars() {
        match c {
            '[' => {
                stack.push(c);
            }
            ']' => {
                if let Some(x) = stack.last() {
                    if *x == '[' {
                        stack.pop();
                        continue;
                    }
                }
                stack.push(c);
            }
            _ => {}
        };
    }

    return (stack.len() as f64 / 4.0).ceil() as i32;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = min_swaps("][][".into());
        assert_eq!(result, 1);
    }

    #[test]
    fn example_two() {
        let result = min_swaps("]]][[[".into());
        assert_eq!(result, 2);
    }

    #[test]
    fn example_three() {
        let result = min_swaps("[]".into());
        assert_eq!(result, 0);
    }

    #[test]
    fn example_four() {
        let result = min_swaps(
            "]]]]][[[[[]]][[][[[]][[]]]]]]]]]]]]]][[[[]][[[[[[[[]][[][][[][[[[[]]][][".into(),
        );
        assert_eq!(result, 7);
    }
}
