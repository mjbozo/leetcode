// LeetCode problem 2966: Divide Array Into Arrays With Max Difference
// https://leetcode.com/problems/divide-array-into-arrays-with-max-difference/description/

fn main() {
    let result = divide_array(vec![1, 3, 4, 8, 7, 9, 3, 5, 1], 2);
    println!("Result = {result:?}");
}

fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    nums.sort_unstable();

    let mut result: Vec<Vec<i32>> = Vec::with_capacity(nums.len() / 3);
    let mut chunks = nums.chunks(3);
    while let Some(c) = chunks.next() {
        if c[2] - c[0] <= k {
            result.push(c.to_vec());
        } else {
            return vec![];
        }
    }

    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = divide_array(vec![1, 3, 4, 8, 7, 9, 3, 5, 1], 2);
        assert_eq!(result, vec![vec![1, 1, 3], vec![3, 4, 5], vec![7, 8, 9]]);
    }

    #[test]
    fn examle_two() {
        let result = divide_array(vec![1, 3, 3, 2, 7, 3], 3);
        assert_eq!(result, Vec::<Vec<i32>>::new());
    }
}
