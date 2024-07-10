// LeetCode problem 2485: Find the Pivot Integer
// https://leetcode.com/problems/find-the-pivot-integer/description/

fn main() {
    let result = pivot_integer(8);
    println!("Result = {result}");
}

fn pivot_integer(mut n: i32) -> i32 {
    let mut left_sum = n * (n + 1) / 2;
    let mut right_sum = n;

    while n >= 0 {
        if left_sum == right_sum {
            return n;
        }

        if left_sum < right_sum {
            break;
        }

        left_sum -= n;
        right_sum += n - 1;
        n -= 1;
    }

    return -1;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = pivot_integer(8);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_two() {
        let result = pivot_integer(1);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_three() {
        let result = pivot_integer(4);
        assert_eq!(result, -1);
    }
}
