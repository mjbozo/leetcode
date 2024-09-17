// LeetCode problem 0179: Largest Number
// https://leetcode.com/problems/largest-number/description/

fn main() {
    let result = largest_number(vec![34323, 3432]);
    println!("Result = {result}");
    let result2 = largest_number(vec![0, 0]);
    println!("Result = {result2}");
}

fn largest_number(nums: Vec<i32>) -> String {
    let mut nums = nums
        .iter()
        .map(|n| String::from(format!("{}", n)))
        .collect::<Vec<String>>();
    nums.sort_by(|a, b| {
        let a_binding = a.to_owned() + b.as_str();
        let mut a_chars = a_binding.chars().peekable();
        let b_binding = b.to_owned() + a.as_str();
        let mut b_chars = b_binding.chars().peekable();

        let mut a_char = '0';
        let mut b_char = '0';
        while a_chars.peek().is_some() || b_chars.peek().is_some() {
            if let Some(a) = a_chars.next() {
                a_char = a;
            }

            if let Some(b) = b_chars.next() {
                b_char = b;
            }

            if a_char != b_char {
                return b_char.cmp(&a_char);
            }
        }

        b.cmp(&a)
    });

    // remove leading zeros
    if nums[0] == "0" {
        return String::from("0");
    }

    return nums.join("");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = largest_number(vec![10, 2]);
        assert_eq!(result, String::from("210"));
    }

    #[test]
    fn example_two() {
        let result = largest_number(vec![3, 30, 34, 5, 9]);
        assert_eq!(result, String::from("9534330"));
    }
}
