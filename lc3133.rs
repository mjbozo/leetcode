// LeetCode problem 3133: Minimum Array End
// https://leetcode.com/problems/minimum-array-end/description/

fn main() {
    let result = min_end(6715154, 7193485);
    println!("Result = {result}");
}

fn min_end(n: i32, x: i32) -> i64 {
    let mut n_sub = n - 1;
    let mut result = x as i64;

    for i in 0..64 {
        if result & (1 << i) == 0 {
            if n_sub & 1 == 1 {
                result ^= 1 << i;
            }
            n_sub = n_sub >> 1;
        }
    }

    return result as i64;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = min_end(3, 4);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_two() {
        let result = min_end(2, 7);
        assert_eq!(result, 15);
    }
}
