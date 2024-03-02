// LeetCode problem 0027: Remove Element
// https://leetcode.com/problems/remove-element/description/

fn main() {
    let result = remove_element(&mut vec![3,2,2,3], 3);
    println!("Result = {result}");
}

fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut i = 0;
    while i < nums.len() {
        while nums[i] == val {
            nums.remove(i);
            if i == nums.len() {
                break;
            }
        }
        i += 1;
    }
    
    return nums.len() as i32;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = remove_element(&mut vec![3,2,2,3], 3);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_two() {
        let result = remove_element(&mut vec![0,1,2,2,3,0,4,2], 2);
        assert_eq!(result, 5);
    }
}
