// LeetCode problem 1672: Richest Customer Wealth
// https://leetcode.com/problems/richest-customer-wealth/description/

fn main() {
    let result = maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]]);
    println!("Result = {result}");
}

fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    let mut richest = 0;
    for account in accounts {
        let mut current = 0;
        for bank in account {
            current += bank;
        }
        richest = richest.max(current);
    }
    return richest;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]]);
        assert_eq!(result, 6)
    }

    #[test]
    fn example_two() {
        let result = maximum_wealth(vec![vec![1, 5], vec![7, 3], vec![3, 5]]);
        assert_eq!(result, 10);
    }
}
