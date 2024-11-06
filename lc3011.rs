// LeetCode problem 3011: Find if Array Can Be Sorted
// https://leetcode.com/problems/find-if-array-can-be-sorted/description/

fn main() {
    let result = can_sort_array(vec![75, 34, 30]);
    println!("Result = {result}");
}

fn can_sort_array(nums: Vec<i32>) -> bool {
    let mut ranges = vec![];

    let mut current = (nums[0], nums[0], nums[0].count_ones());
    for num in nums.into_iter() {
        let num_ones = num.count_ones();
        if num_ones != current.2 {
            ranges.push((current.0, current.1));
            current = (num, num, num_ones);
            continue;
        }

        if num < current.0 {
            current.0 = num;
        }

        if num > current.1 {
            current.1 = num;
        }
    }

    ranges.push((current.0, current.1));

    println!("{:?}", ranges);

    for w in ranges.windows(2) {
        if w[0].1 > w[1].0 {
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
        let result = can_sort_array(vec![8, 4, 2, 30, 15]);
        assert!(result);
    }

    #[test]
    fn example_two() {
        let result = can_sort_array(vec![1, 2, 3, 4, 5]);
        assert!(result);
    }

    #[test]
    fn example_three() {
        let result = can_sort_array(vec![3, 16, 8, 4, 2]);
        assert!(!result);
    }
}
