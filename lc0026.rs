// LeetCode problem 0026: Remove Duplicates from Sorted Array
// https://leetcode.com/problems/remove-duplicates-from-sorted-array/description/

fn main() {
    let result = remove_duplicates(&mut vec![1,1,2]);
    println!("Result = {result}");
}

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut i = 1;
    while i < nums.len() {
        if nums[i] == nums[i - 1] {
            nums.remove(i);
        } else {
            i += 1;
        }
    }

    println!("{nums:?}");
    return nums.len() as i32;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = remove_duplicates(&mut vec![1,1,2]);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_two() {
        let result = remove_duplicates(&mut vec![0,0,1,1,1,2,2,3,3,4]);
        assert_eq!(result, 5);
    }
}
