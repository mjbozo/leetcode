// LeetCode problem 1962: Remove Stones to Minimize the Total
// https://leetcode.com/problems/remove-stones-to-minimize-the-total/description/

fn main() {
    let result = min_stone_sum(vec![5, 4, 9], 2);
    println!("Result = {result}");
}

fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
    let mut heap = std::collections::BinaryHeap::from(piles);

    for _ in 0..k {
        let value = heap.pop().unwrap();
        heap.push((value + 1) / 2);
    }

    return heap.iter().sum::<i32>();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = min_stone_sum(vec![5, 4, 9], 2);
        assert_eq!(result, 12);
    }

    #[test]
    fn example_two() {
        let result = min_stone_sum(vec![4, 3, 6, 7], 3);
        assert_eq!(result, 12);
    }
}
