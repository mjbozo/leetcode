// LeetCode problem 0746: Min Cost Climbing Stairs
// https://leetcode.com/problems/min-cost-climbing-stairs/description/

fn main() {
    let result = min_cost_climbing_stairs(vec![10, 15, 20]);
    println!("Result = {result}");
}

fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let n = cost.len();
    let mut dp = vec![0; n + 1];
    dp[0] = 0;
    dp[1] = 0;

    for i in 2..=n {
        dp[i] = std::cmp::min(dp[i - 2] + cost[i - 2], dp[i - 1] + cost[i - 1]);
    }

    return *dp.last().unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = min_cost_climbing_stairs(vec![10, 15, 20]);
        assert_eq!(result, 15);
    }

    #[test]
    fn example_two() {
        let result = min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]);
        assert_eq!(result, 6);
    }
}
