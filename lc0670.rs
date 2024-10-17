// LeetCode problem 0670: Maximum Swap
// https://leetcode.com/problems/maximum-swap/description/

fn main() {
    let result = maximum_swap(2736);
    println!("Result = {result}");
}

fn maximum_swap(num: i32) -> i32 {
    let mut number = num;
    let mut digits = vec![];

    while number > 0 {
        digits.push(number % 10);
        number = number / 10;
    }

    digits = digits.into_iter().rev().collect::<Vec<_>>();

    let mut increase_found = false;
    let mut max_index = 0;
    for i in 1..digits.len() {
        if !increase_found && digits[i] > digits[i - 1] {
            increase_found = true;
            max_index = i;
        }

        if increase_found && digits[i] >= digits[max_index] {
            max_index = i;
        }
    }

    if !increase_found {
        return num;
    }

    for i in 0..digits.len() {
        if digits[i] < digits[max_index] {
            let temp = digits[max_index];
            digits[max_index] = digits[i];
            digits[i] = temp;
            break;
        }
    }

    digits = digits.into_iter().rev().collect::<Vec<_>>();
    let mut result = 0;
    let mut multiplier = 1;
    for x in digits {
        result += x * multiplier;
        multiplier *= 10;
    }

    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = maximum_swap(2736);
        assert_eq!(result, 7236);
    }

    #[test]
    fn example_two() {
        let result = maximum_swap(9973);
        assert_eq!(result, 9973);
    }
}
