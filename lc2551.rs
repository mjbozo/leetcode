// LeetCode problem 2551: Put Marbles in Bags
// https://leetcode.com/problems/put-marbles-in-bags/description/

fn main() {
    let result = put_marbles(vec![1, 3, 5, 1], 2);
    println!("Result = {result}");
}

fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
    let mut pair_sums = Vec::with_capacity(k as usize - 1);
    for i in 1..weights.len() {
        pair_sums.push(weights[i] + weights[i - 1]);
    }
    pair_sums.sort_unstable();

    let mut min_sum = 0;
    for i in 0..k - 1 {
        min_sum += pair_sums[i as usize] as i64;
    }

    let mut max_sum = 0;
    let n = pair_sums.len();
    for i in (n - (k as usize - 1))..n {
        max_sum += pair_sums[i] as i64;
    }

    return max_sum - min_sum;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = put_marbles(vec![1, 3, 5, 1], 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_two() {
        let result = put_marbles(vec![1, 3], 2);
        assert_eq!(result, 0);
    }
}
