// LeetCode problem 2601: Prime Subtraction Operation
// https://leetcode.com/problems/prime-subtraction-operation/description/

fn main() {
    let result = prime_sub_operation(vec![4, 9, 6, 10]);
    println!("Result = {result}");
}

fn prime_sub_operation(nums: Vec<i32>) -> bool {
    let mut nums = nums;
    let primes = sieve_of_eratosthenes(1000);

    let n = nums.len();
    for i in (0..n - 1).rev() {
        if nums[i] >= nums[i + 1] {
            let mut found = false;
            for p in primes.iter() {
                if *p > (nums[i] - nums[i + 1]) {
                    if *p >= nums[i] {
                        return false;
                    }

                    found = true;
                    nums[i] -= *p;
                    break;
                }
            }

            if !found {
                return false;
            }
        }
    }

    return true;
}

fn sieve_of_eratosthenes(n: usize) -> Vec<i32> {
    let mut primes = vec![true; n];
    primes[0] = false;
    primes[1] = false;

    let limit = (n as f64).sqrt() as usize;
    for i in 2..limit {
        if primes[i] {
            for j in ((i * i)..n).step_by(i) {
                primes[j] = false;
            }
        }
    }

    return primes
        .iter()
        .enumerate()
        .filter(|v| *v.1)
        .map(|v| v.0 as i32)
        .collect::<Vec<_>>();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = prime_sub_operation(vec![4, 9, 6, 10]);
        assert!(result);
    }

    #[test]
    fn example_two() {
        let result = prime_sub_operation(vec![6, 8, 11, 12]);
        assert!(result);
    }

    #[test]
    fn example_three() {
        let result = prime_sub_operation(vec![5, 8, 3]);
        assert!(!result);
    }
}
