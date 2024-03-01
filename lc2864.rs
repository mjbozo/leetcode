// LeetCode problem 2864: Maximum Odd Binary Number
// https://leetcode.com/problems/maximum-odd-binary-number/description/

fn main() {
    let result = maximum_odd_binary_number(String::from("010"));
    println!("Result = {result}");
}

fn maximum_odd_binary_number(s: String) -> String {
    let s_len = s.len();
    let num_ones = s.chars().fold(0, |acc, v| if v == '1' { acc + 1 } else { acc });

    let mut max_binary = String::new();
    for _ in 0..(num_ones - 1) {
        max_binary.push('1');
    }
    
    for _ in 0..(s_len - num_ones) {
        max_binary.push('0');
    }

    max_binary.push('1');
    return max_binary;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = maximum_odd_binary_number(String::from("010"));
        assert_eq!(result, String::from("001"));
    }

    #[test]
    fn example_two() {
        let result = maximum_odd_binary_number(String::from("0101"));
        assert_eq!(result, String::from("1001"));
    }
}
