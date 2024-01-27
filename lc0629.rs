// LeetCode problem 0629: K Inverse Pairs Array
// https://leetcode.com/problems/k-inverse-pairs-array/description/

fn main() {
    let result = k_inverse_pairs(3, 2);
    println!("Result = {result}");
}

fn k_inverse_pairs(n: i32, k: i32) -> i32 {
    let modulo = 1000000007;
    let k = k as usize;
    let n = n as usize;

    let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; k + 1];
    
    for row in 0..=k {
        for col in 0..n {
            if row == 0 {
                dp[row][col] = 1;
                continue;
            }

            if col == 0 {
                dp[row][col] = 0;
                continue;
            }

            let mut current = 0;
            let mut offset = 0;
            while offset <= col && row.checked_sub(offset).is_some() {
                current = (current + dp[row - offset][col - 1]) % modulo;
                offset += 1;
            }

            dp[row][col] = current;
        }
    }

    return dp[k][n - 1];
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = k_inverse_pairs(3, 0);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_two() {
        let result = k_inverse_pairs(3, 1);
        assert_eq!(result, 2);
    }
}
