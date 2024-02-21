// LeetCode problem 0022: Generate Parentheses
// https://leetcode.com/problems/generate-parentheses/description/

use std::collections::HashSet;

fn main() {
    let result = generate_parentheses(8);
    println!("Result = {result:?}");
}

fn generate_parentheses(n: i32) -> Vec<String> {
    let mut dp = vec![vec![String::from("()")]];

    for i in 1..n {
        let mut combos = HashSet::new();

        let prev = &dp[i as usize - 1];
        for s in prev {
            combos.insert(String::from("(") + &s + ")");
        }

        let mut left = 0;
        let mut right = dp.len() - 1;
        while left <= right {
            for l in &dp[left] {
                for r in &dp[right] {
                    combos.insert(l.to_owned() + r);
                    combos.insert(r.to_owned() + l);
                }
            }

            if right == 0 {
                break;
            }

            left += 1;
            right -= 1;
        }

        dp.push(combos.into_iter().collect::<Vec<_>>());
    }

    return dp.last().unwrap_or(&vec![]).to_vec();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = generate_parentheses(3);
        assert_eq!(result, vec![String::from("((()))"), String::from("(()())"), String::from("(())()"),
            String::from("()(())"), String::from("()()()")]);
    }

    #[test]
    fn example_two() {
        let result = generate_parentheses(1);
        assert_eq!(result, vec![String::from("()")]);
    }
}
