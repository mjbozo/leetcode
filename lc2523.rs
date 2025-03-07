// LeetCode problem 2523: Closest Prime Numbers in Range
// https://leetcode.com/problems/closest-prime-numbers-in-range/description/

fn main() {
    let result = closest_primes(1, 1000000);
    println!("Result = {result:?}");
}

fn closest_primes(left: i32, right: i32) -> Vec<i32> {
    let primes = sieve_of_eratosthenes(right, left);
    let mut closest = vec![-1, -1];
    let mut closest_set = false;
    for w in primes.windows(2) {
        if w[1] - w[0] < closest[1] - closest[0] || !closest_set {
            closest[0] = w[0];
            closest[1] = w[1];
            closest_set = true;
        }
    }

    return closest;
}

fn sieve_of_eratosthenes(n: i32, min: i32) -> Vec<i32> {
    let mut primes = vec![true; n as usize + 1];
    primes[1] = false;
    let sqrt = (n as f64).sqrt() as usize;
    for i in 2..=sqrt {
        if primes[i] {
            for j in (i * i..=n as usize).step_by(i) {
                primes[j] = false
            }
        }
    }

    let mut nums = Vec::with_capacity(n as usize);
    for (i, p) in primes.iter().enumerate() {
        if *p && i >= min as usize {
            nums.push(i as i32);
        }
    }

    return nums;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = closest_primes(10, 19);
        assert_eq!(result, vec![11, 13]);
    }

    #[test]
    fn example_two() {
        let result = closest_primes(4, 6);
        assert_eq!(result, vec![-1, -1]);
    }
}
