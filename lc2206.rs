// LeetCode problem 2206: Divide Array Into Equal Pairs
// https://leetcode.com/problems/divide-array-into-equal-pairs/description/

fn main() {
    let result = divide_array(vec![3, 2, 3, 2, 2, 2]);
    println!("Result = {result}");
}

fn divide_array(nums: Vec<i32>) -> bool {
    let mut counter = std::collections::HashMap::new();
    for num in nums {
        counter.entry(num).and_modify(|v| *v += 1).or_insert(1);
    }

    for (k, v) in counter {
        if v % 2 == 1 {
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
        let result = divide_array(vec![3, 2, 3, 2, 2, 2]);
        assert!(result);
    }

    #[test]
    fn example_two() {
        let result = divide_array(vec![1, 2, 3, 4]);
        assert!(!result);
    }
}
