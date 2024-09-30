// LeetCode problem 0509: Fibonacci Number
// https://leetcode.com/problems/fibonacci-number/description/

fn main() {
    let result = fib(10);
    println!("Result = {result}");
}

fn fib(n: i32) -> i32 {
    let mut dp = vec![0; n as usize + 1];
    dp[0] = 0;
    dp[1] = 1;

    for i in 2..=n {
        let i = i as usize;
        dp[i] = dp[i - 1] + dp[i - 2];
    }

    return *dp.last().unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = fib(2);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_two() {
        let result = fib(3);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_three() {
        let result = fib(4);
        assert_eq!(result, 3);
    }
}
