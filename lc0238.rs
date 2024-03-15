// LeetCode problem 0238: Product of Array Except Self
// https://leetcode.com/problems/product-of-array-except-self/description/

fn main() {
    let result = product_except_self(vec![1,2,3,4]);
    println!("Result = {result:?}");
}

fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut left_products = Vec::with_capacity(len);
    let mut right_products = Vec::with_capacity(len);
    left_products.push(1);
    right_products.push(1);
    
    for i in 1..len {
        left_products.push(left_products[i-1]*nums[i-1]);
        right_products.push(right_products[i-1]*nums[len-i]);
    }

    let mut products = Vec::with_capacity(len);
    for n in 0..len {
        products.push(left_products[n] * right_products[len-n-1]);
    }

    return products;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = product_except_self(vec![1,2,3,4]);
        assert_eq!(result, vec![24, 12, 8, 6]);
    }

    #[test]
    fn example_two() {
        let result = product_except_self(vec![-1,1,0,-3,3]);
        assert_eq!(result, vec![0,0,9,0,0]);
    }
}
