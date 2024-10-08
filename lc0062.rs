// LeetCode problem 0062: Unique Paths
// https://leetcode.com/problems/unique-paths/description/

fn main() {
    let result = unique_paths(3, 7);
    println!("Result = {result}");
}

fn unique_paths(m: i32, n: i32) -> i32 {
    let mut dp = vec![vec![0; n as usize]; m as usize];

    for x in 0..n as usize {
        dp[0][x] = 1;
    }

    for y in 0..m as usize {
        dp[y][0] = 1;
    }

    for j in 1..m as usize {
        for i in 1..n as usize {
            dp[j][i] = dp[j - 1][i] + dp[j][i - 1];
        }
    }

    return *dp.last().unwrap().last().unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = unique_paths(3, 7);
        assert_eq!(result, 28);
    }

    #[test]
    fn example_two() {
        let result = unique_paths(3, 2);
        assert_eq!(result, 3);
    }
}
