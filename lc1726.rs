// LeetCode problem 1726: Tuple with Same Product
// https://leetcode.com/problems/tuple-with-same-product/description/

fn main() {
    let result = tuple_same_product(vec![1, 2, 3, 4, 6, 12]);
    println!("Result = {result}");
}

fn tuple_same_product(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut nums = nums;
    nums.sort_unstable();

    let mut product_counts = std::collections::HashMap::with_capacity(n * (n - 1) / 2);

    for i in 0..n - 1 {
        for j in i + 1..n {
            product_counts
                .entry(nums[i] * nums[j])
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
    }

    let mut total = 0;
    for (_, v) in product_counts {
        total += v * (v - 1) / 2;
    }

    return total * 8;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = tuple_same_product(vec![2, 3, 4, 6]);
        assert_eq!(result, 8);
    }

    #[test]
    fn example_two() {
        let result = tuple_same_product(vec![1, 2, 4, 5, 10]);
        assert_eq!(result, 16);
    }
}
