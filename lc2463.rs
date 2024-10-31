// LeetCode problem 2463: Minimum Total Distance Traveled
// https://leetcode.com/problems/minimum-total-distance-traveled/description/

fn main() {
    let result = minimum_total_distance(vec![0, 4, 6], vec![vec![2, 2], vec![6, 2]]);
    println!("Result = {result}");
}

fn minimum_total_distance(robot: Vec<i32>, factory: Vec<Vec<i32>>) -> i64 {
    let r = robot.len();
    let mut robots = robot;
    robots.sort_unstable();

    let mut factories = factory;
    factories.sort_unstable_by_key(|v| v[0]);

    let mut factory_positions = vec![];
    for x in &factories {
        for _ in 0..x[1] {
            factory_positions.push(x[0]);
        }
    }
    let f = factory_positions.len();

    let mut dp = vec![vec![0; f + 1]; r + 1];

    for i in 0..r {
        dp[i][f] = 1e12 as i64;
    }

    for i in (0..r).rev() {
        for j in (0..f).rev() {
            let assign = (robots[i] - factory_positions[j]).abs() as i64 + dp[i + 1][j + 1];
            let skip = dp[i][j + 1];
            dp[i][j] = std::cmp::min(assign, skip);
        }
    }

    return dp[0][0];
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = minimum_total_distance(vec![0, 4, 6], vec![vec![2, 2], vec![6, 2]]);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_two() {
        let result = minimum_total_distance(vec![1, -1], vec![vec![-2, 1], vec![2, 1]]);
        assert_eq!(result, 2);
    }
}
