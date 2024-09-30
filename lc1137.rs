// LeetCode problem 1137: N-th Tribonacci Number
// https://leetcode.com/problems/n-th-tribonacci-number/description/

fn main() {
    let result = tribonacci(4);
    println!("Result = {result}");
}

fn tribonacci(n: i32) -> i32 {
    if n == 0 || n == 1 {
        return n;
    }

    if n == 2 {
        return 1;
    }

    let mut dp = vec![0; n as usize + 1];
    dp[0] = 0;
    dp[1] = 1;
    dp[2] = 1;

    for i in 3..=n {
        let i = i as usize;
        dp[i] = dp[i - 3] + dp[i - 2] + dp[i - 1]
    }

    return *dp.last().unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = tribonacci(4);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_two() {
        let result = tribonacci(25);
        assert_eq!(result, 1389537);
    }
}
