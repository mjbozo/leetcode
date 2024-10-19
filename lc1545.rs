// LeetCode problem 1545: Find Kth Bit in Nth Binary String
// https://leetcode.com/problems/find-kth-bit-in-nth-binary-string/description/

fn main() {
    let result = find_kth_bit(4, 14);
    println!("Result = {result}");
}

fn find_kth_bit(n: i32, k: i32) -> char {
    let mut dp = vec![String::new(); n as usize];

    dp[0] = String::from("0");
    if n == 1 {
        return dp[0].chars().nth(k as usize - 1).unwrap();
    }

    dp[1] = String::from("011");
    if n == 2 {
        return dp[1].chars().nth(k as usize - 1).unwrap();
    }

    dp[2] = String::from("0111001");
    for i in 3..n as usize {
        let prev_mid = (dp[i - 1].len() + 1) / 2;
        dp[i] = dp[i - 1].clone() + "1" + &dp[i - 2] + "0" + &dp[i - 1][prev_mid..];
    }

    return dp.last().unwrap().chars().nth(k as usize - 1).unwrap();
}

fn find_kth_bit_brute(n: i32, k: i32) -> char {
    let mut s = String::from("0");
    for _ in 1..n {
        let mut invert_reversed = String::new();
        for c in s.chars() {
            match c {
                '0' => invert_reversed = String::from("1") + &invert_reversed,
                '1' => invert_reversed = String::from("0") + &invert_reversed,
                _ => {}
            };
        }
        s = s + "1" + &invert_reversed;
        println!("{s}");
    }
    return s.chars().nth(k as usize - 1).unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = find_kth_bit(3, 1);
        assert_eq!(result, '0');
    }

    #[test]
    fn example_two() {
        let result = find_kth_bit(4, 11);
        assert_eq!(result '1');
    }
}
