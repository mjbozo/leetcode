// LeetCode problem 0070: Climbing Stairs
// https://leetcode.com/problems/climbing-stairs/description/

fn main() {
    let result = climb_stairs(45);
    println!("Result = {result}");
}

fn climb_stairs(n: i32) -> i32 {
    let mut dp = vec![1, 2];

    if n == 1 {
        return 1;
    }

    if n == 2 {
        return 2;
    }

    for i in 3..n {
        dp.push(dp[i as usize - 2] + dp[i as usize - 3]);
    }

    let dp_len = dp.len();
    return dp[dp_len - 1] + dp[dp_len - 2];
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = climb_stairs(2);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_two() {
        let result = climb_stairs(3);
        assert_eq!(result, 3);
    }
}
