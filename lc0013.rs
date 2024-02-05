// LeetCode problem 0013: Roman to Integer
// https://leetcode.com/problems/roman-to-integer/description/

use std::collections::HashMap;

fn main() {
    let result = roman_to_int(String::from("III"));
    println!("Result = {result}");
}

fn roman_to_int(s: String) -> i32 {
    let numerals = HashMap::from([
        (b'I', 1), (b'V', 5), (b'X', 10), (b'L', 50), (b'C', 100), (b'D', 500), (b'M', 1000)
    ]);

    let bytes = s.bytes().collect::<Vec<_>>();
    let mut number = 0;
    let mut i = 0;
    while i < bytes.len() {
        let value = numerals.get(&bytes[i]).expect("Char is invalid");
        let next_char = bytes.get(i + 1);
        let next_value = if let Some(ch) = next_char {
            numerals.get(&ch).expect("Char is invalid")
        } else {
            &0
        };

        if value < next_value {
            number += next_value - value;
            println!("Value = {value}, Next = {next_value}, Number = {number}");
            i += 1;
        } else {
            number += value;
        }

        i += 1;
    }

    return number;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = roman_to_int(String::from("III"));
        assert_eq!(result, 3);
    }

    #[test]
    fn example_two() {
        let result = roman_to_int(String::from("LVIII"));
        assert_eq!(result, 58);
    }

    #[test]
    fn example_three() {
        let result = roman_to_int(String::from("MCMXCIV"));
        assert_eq!(result, 1994);
    }
}
