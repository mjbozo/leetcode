// LeetCode problem 0023: Merge K Sorted Lists
// https://leetcode.com/problems/merge-k-sorted-lists/description/

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

fn main() {
    let result = merge_k_lists(vec![build_list(vec![1, 4, 5]), build_list(vec![1, 3, 4]), build_list(vec![2, 6])]);
    println!("Result = {result:?}");
}

fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut combined = vec![];
    for mut list in lists {
        let mut current = &mut list;
        while current.is_some() {
            let v = current.clone().unwrap().val;
            combined.push(v);
            current = &mut current.as_mut().unwrap().next;
        }
    }

    combined.sort_unstable();
    let list = build_list(combined);
    return list;
}

fn build_list(vals: Vec<i32>) -> Option<Box<ListNode>> {
    if vals.len() == 0 {
        return None;
    }

    let mut node = ListNode::new(vals[0]);
    node.next = build_list(vals[1..].to_vec());

    return Some(Box::new(node));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = merge_k_lists(vec![build_list(vec![1, 4, 5]), build_list(vec![1, 3, 4]), build_list(vec![2, 6])]);
        assert_eq!(result, build_list(1, 1, 2, 3, 4, 4, 5, 6));
    }

    #[test]
    fn example_two() {
        let result = merge_k_lists(vec![]);
        assert_eq!(result, None);
    }

    #[test]
    fn example_three() {
        let result = merge_k_lists(vec![build_list(vec![])]);
        assert_eq!(result, None);
    }
}
