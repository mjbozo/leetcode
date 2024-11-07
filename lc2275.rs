// LeetCode problem 2275: Largest Combination With Bitwise AND Greater Than Zero
// https://leetcode.com/problems/largest-combination-with-bitwise-and-greater-than-zero/description/

fn main() {
    let result = largest_combination(vec![16, 17, 71, 62, 12, 24, 14]);
    println!("Result = {result}");
}

fn largest_combination(candidates: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut bits = vec![0; 24];
    for c in candidates {
        let mut n = c;
        let mut i = 0;
        while n > 0 {
            if n & 1 == 1 {
                bits[i] += 1;
                if bits[i] > max {
                    max = bits[i];
                }
            }

            i += 1;
            n = n >> 1;
        }
    }

    return max;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = largest_combination(vec![16, 17, 71, 62, 12, 24, 14]);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_two() {
        let result = largest_combination(vec![8, 8]);
        assert_eq!(result, 2);
    }
}
