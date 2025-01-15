// LeetCode problem 2429: Minimize XOR
// https://leetcode.com/problems/minimize-xor/description/

fn main() {
    let result = minimize_xor(3, 5);
    println!("Result = {result}");
}

fn minimize_xor(num1: i32, num2: i32) -> i32 {
    let n1_ones = num1.count_ones() as i32;
    let n2_ones = num2.count_ones() as i32;

    if n2_ones == n1_ones {
        return num1;
    }

    let mut result = num1;
    let n1_greater = n1_ones > n2_ones;
    let n_diff = (n1_ones - n2_ones).abs();
    let mut bits_flipped = 0;
    let mut i = 0;

    while bits_flipped != n_diff {
        let bit_value = num1 & 1 << i;
        if (n1_greater && bit_value != 0) || (!n1_greater && bit_value == 0) {
            result ^= 1 << i;
            bits_flipped += 1;
        }
        i += 1;
    }

    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = minimize_xor(3, 5);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_two() {
        let result = minimize_xor(1, 12);
        assert_eq!(result, 3);
    }
}
