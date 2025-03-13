// LeetCode problem 3356: Zero Array Transformation II
// https://leetcode.com/problems/zero-array-transformation-ii/description/

fn main() {
    let result = min_zero_array(
        vec![2, 0, 2],
        vec![vec![0, 2, 1], vec![0, 2, 1], vec![1, 1, 3]],
    );
    println!("Result = {result}");
}

fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
    let mut diff = vec![0; nums.len() + 1];
    diff[0] = nums[0];
    for i in 1..nums.len() {
        diff[i] = nums[i] - nums[i - 1];
    }

    if is_zero_array(diff.clone()) {
        return 0;
    }

    let mut left = 0 as isize;
    let mut right = queries.len() as isize - 1;
    let mut k = queries.len() as isize;

    while left <= right {
        let mid = (left + right) / 2;
        // apply queries up to mid, check if zero array
        let mut current_diff = diff.clone();
        for i in 0..=mid as usize {
            current_diff[queries[i][0] as usize] -= queries[i][2];
            current_diff[queries[i][1] as usize + 1] += queries[i][2];
        }

        if is_zero_array(current_diff) {
            k = mid;
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    return if left >= queries.len() as isize {
        -1
    } else {
        k as i32 + 1
    };
}

fn is_zero_array(arr: Vec<i32>) -> bool {
    let mut ps = vec![0; arr.len() + 1];
    ps[0] = arr[0];
    if ps[0] > 0 {
        return false;
    }

    for i in 1..arr.len() - 1 {
        ps[i] = ps[i - 1] + arr[i];
        if ps[i] > 0 {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = min_zero_array(
            vec![2, 0, 2],
            vec![vec![0, 2, 1], vec![0, 2, 1], vec![1, 1, 3]],
        );
        assert_eq!(result, 2);
    }

    #[test]
    fn example_two() {
        let result = min_zero_array(vec![4, 3, 2, 1], vec![vec![1, 3, 2], vec![0, 2, 1]]);
        assert_eq!(result, -1);
    }
}
