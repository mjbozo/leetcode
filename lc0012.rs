// LeetCode problem 0012: Integer to Roman
// https://leetcode.com/problems/integer-to-roman/description/

use std::collections::HashMap;

fn main() {
    let result = int_to_roman(69);
    println!("Result = {result}");
}

fn int_to_roman(num: i32) -> String {
    let ones = vec!["I", "X", "C", "M"];
    let fives = vec!["V", "L", "D"];
    let prefixes = HashMap::from([
        (4, "IV"),
        (9, "IX"),
        (40, "XL"),
        (90, "XC"),
        (400, "CD"),
        (900, "CM"),
    ]);

    let mut n = num;
    let mut i = 0;
    let mut roman: Vec<&str> = vec![];
    while n > 0 {
        let rem = n % 10;
        n /= 10;

        if (rem + 1) % 5 == 0 {
            let s = prefixes.get(&(rem * 10_i32.pow(i))).unwrap_or(&"");
            roman.push(s);
            i += 1;
            continue;
        }

        for _ in 0..(rem % 5) {
            let s = ones[i as usize];
            roman.push(s);
        }

        if rem >= 5 {
            let s = fives[i as usize];
            roman.push(s);
        }

        i += 1;
    }

    return roman.into_iter().rev().collect::<String>();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = int_to_roman(3);
        assert_eq!(result, String::from("III"));
    }

    #[test]
    fn example_two() {
        let result = int_to_roman(58);
        assert_eq!(result, String::from("LVIII"));
    }

    #[test]
    fn example_three() {
        let result = int_to_roman(1994);
        assert_eq!(result, "MCMXCIV");
    }
}
