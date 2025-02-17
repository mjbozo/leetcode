// LeetCode problem 1718: Construct the Lexicographically Largest Valid Sequence
// https://leetcode.com/problems/construct-the-lexicographically-largest-valid-sequence/description/

fn main() {
    let result = construct_distanced_sequence(5);
    println!("Result = {result:?}");
}

fn construct_distanced_sequence(n: i32) -> Vec<i32> {
    let mut res = vec![0; (2 * n - 1) as usize];
    let mut used = vec![false; (n + 1) as usize];

    fill(0, n, &mut res, &mut used);

    return res;
}

fn fill(current: i32, n: i32, mut nums: &mut Vec<i32>, mut used: &mut Vec<bool>) -> bool {
    let s = nums.len() as i32;
    if current == s {
        return true;
    }

    if nums[current as usize] != 0 {
        return fill(current + 1, n, &mut nums, &mut used);
    }

    for x in (1..=n).rev() {
        if used[x as usize] {
            continue;
        }

        used[x as usize] = true;
        nums[current as usize] = x;

        if x == 1 {
            if fill(current + 1, n, &mut nums, &mut used) {
                return true;
            }
        } else if current + x < s && nums[(current + x) as usize] == 0 {
            nums[(current + x) as usize] = x;
            if fill(current + 1, n, &mut nums, &mut used) {
                return true;
            }
            nums[(current + x) as usize] = 0;
        }

        nums[current as usize] = 0;
        used[x as usize] = false;
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
