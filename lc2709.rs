// LeetCode problem 2709: Greatest Common Divisor Traversal
// https://leetcode.com/problems/greatest-common-divisor-traversal/description/

use std::collections::HashSet;

struct UnionFind {
    parents: Vec<usize>
}
impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parents: (0..n).collect()
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while x != self.parents[x] {
            x = self.parents[x];
            self.parents[x] = self.parents[self.parents[x]];
        }
        return x;
    }

    fn union(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        if x != y {
            self.parents[x] = y;
        }
    }
}

fn main() {
    let result = can_traverse_all_pairs(vec![33,33,10]);
    println!("Result = {result}");
}

fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
    let n = nums.len();
    if n == 1 {
        return true;
    }

    let max = *nums.iter().max().unwrap();
    let is_prime = sieve_of_eratosthenes(max as usize + 1);
    let primes: Vec<i32> = is_prime.iter().enumerate().filter_map(|(idx, val)| val.then(|| idx as i32)).collect();

    let mut uf = UnionFind::new(max as usize + 1);
    for i in 0..n {
        if nums[i] == 1 {
            return false;
        }

        if is_prime[nums[i] as usize] {
            continue;
        }

        for j in 0..primes.len() {
            if primes[j] > nums[i] {
                break;
            }

            if nums[i] % primes[j] == 0 {
                uf.union(nums[i] as usize, primes[j] as usize);
            }
        }
    }

    let graphs = nums.into_iter().map(|v| uf.find(v as usize)).collect::<HashSet<usize>>();
    return graphs.len() == 1;
}

fn sieve_of_eratosthenes(n: usize) -> Vec<bool> {
    let mut is_prime = vec![true; n];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..n {
        if is_prime[i] {
            for j in ((i * i)..n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    return is_prime;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = can_traverse_all_pairs(vec![2,3,6]);
        assert!(result);
    }

    #[test]
    fn example_two() {
        let result = can_traverse_all_pairs(vec![3,9,5]);
        assert!(!result);
    }

    #[test]
    fn example_three() {
        let result = can_traverse_all_pairs(vec![4,3,12,8]);
        assert!(result);
    }
}
