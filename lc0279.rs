// LeetCode problem 0279: Perfect Squares
// https://leetcode.com/problems/perfect-squares/description/

fn main() {
    let result = num_squares(10000);
    println!("Result = {result}");
}

fn num_squares(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![0; n];
    let mut next_base = 2;

    dp[0] = 1;

    for i in 1..n {
        let num = i + 1;

        if num == next_base * next_base {
            dp[i] = 1;
            next_base += 1;
            continue;
        }

        let mut left = 0 as usize;
        let mut right = i - 1;
        let mut lowest = i32::MAX;
        loop {
            if dp[left] == 1 && dp[right] == 1 {
                lowest = 2;
                break;
            }

            let combined = dp[left] + dp[right];
            if combined < lowest {
                lowest = combined;
            }

            if right - left <= 1 {
                break;
            }

            left += 1;
            right -= 1;
        }

        dp[i] = lowest;
    }

    println!("{dp:?}");
    return dp.pop().unwrap_or(0);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = num_squares(12);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_two() {
        let result = num_squares(13);
        assert_eq!(result, 2);
    }
}
