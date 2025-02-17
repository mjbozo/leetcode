// LeetCode problem 1718: Construct the Lexicographically Largest Valid Sequence
// https://leetcode.com/problems/construct-the-lexicographically-largest-valid-sequence/description/

use std::collections::HashSet;

fn main() {
    let result = construct_distanced_sequence(5);
    println!("Result = {result:?}");
}

fn construct_distanced_sequence(n: i32) -> Vec<i32> {
    let mut res = vec![0; (2 * n - 1) as usize];
    let mut used = HashSet::new();

    fill(0, n, &mut res, &mut used);

    return res;
}

fn fill(i: i32, n: i32, mut nums: &mut Vec<i32>, mut used: &mut HashSet<i32>) -> bool {
    if i == nums.len() as i32 {
        return true;
    }

    for x in (1..=n).rev() {
        if used.contains(&x) {
            continue;
        }

        if x == 1 && nums[i as usize] == 0 {
            nums[i as usize] = x;
            used.insert(x);

            if fill(i + 1, n, &mut nums, &mut used) {
                return true;
            } else {
                used.remove(&x);
            }
        } else if nums[i as usize] == 0 && nums[(i + x) as usize] == 0 {
            nums[i as usize] = x;
            nums[(i + x) as usize] = x;
            used.insert(x);

            if fill(i + 1, n, &mut nums, &mut used) {
                return true;
            } else {
                used.remove(&x);
            }
        }
    }

    return false;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = construct_distanced_sequence(3);
        assert_eq!(result, vec![3, 1, 2, 3, 2]);
    }

    #[test]
    fn example_two() {
        let result = construct_distanced_sequence(5);
        assert_eq!(result, vec![5, 3, 1, 4, 3, 5, 2, 4, 2]);
    }
}
