// LeetCode problem 0008: String to Integer (atoi)
// https://leetcode.com/problems/string-to-integer-atoi/description/

fn main () {
    let result = my_atoi(String::from("  0000000000012345678"));
    println!("Result = {result}");
}

fn my_atoi(s: String) -> i32 {
    let mut nums: Vec<i32> = vec![];
    let mut whitespace_skipped = false;
    let mut sign_read = false;
    let mut negative = false;
    for c in s.chars() {
        if !whitespace_skipped && c == ' ' {
            continue;
        }

        if !whitespace_skipped && c != ' ' {
            whitespace_skipped = true;
        }

        if !sign_read {
            sign_read = true;
            if c == '-' {
                negative = true;
                continue;
            }

            if c == '+' {
                continue;
            }
        }

        if !c.is_digit(10) {
            break;
        }

        nums.push(c as i32 - b'0' as i32);
    }

    let mut result: i32 = 0;
    for (index, num) in nums.iter().enumerate() {
        if *num == 0 && result == 0 {
            continue;
        }

        let power = 10_i32.checked_pow((nums.len() - index - 1) as u32);
        if power.is_none() {
            if negative {
                return i32::MIN;
            } else {
                return i32::MAX;
            }
        }

        let magnitude = power.unwrap();
        let addition = num.checked_mul(magnitude);
        if addition.is_none() {
            if negative {
                return i32::MIN
            } else {
                return i32::MAX;
            }
        }

        let sum = result.checked_add(addition.unwrap());
        if sum.is_none() {
            if negative {
                return i32::MIN;
            } else {
                return i32::MAX;
            }
        }

        result = sum.unwrap();
    }

    if negative {
        result *= -1;
    }

    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = my_atoi(String::from("42"));
        assert_eq!(result, 42);
    }

    #[test]
    fn example_two() {
        let result = my_atoi(String::from("   -42"));
        assert_eq!(result, -42);
    }

    #[test]
    fn example_three() {
        let result = my_atoi(String::from("4193 with words"));
        assert_eq!(result, 4193);
    }
}
