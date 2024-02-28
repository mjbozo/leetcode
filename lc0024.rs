// LeetCode problem 0024: Swap Nodes in Pairs
// https://leetcode.com/problems/swap-nodes-in-pairs/description/

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

    fn from(vals: Vec<i32>) -> Option<Box<Self>> {
        if vals.len() == 0 {
            return None;
        }

        let mut node = ListNode::new(vals[0]);
        node.next = Self::from(vals[1..].to_vec());

        return Some(Box::new(node));
    }
}

fn main() {
    let result = swap_pairs(ListNode::from(vec![1, 2, 3, 4]));
    println!("Result = {result:?}");
}

fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if let Some(mut current) = head {
        if let Some(mut next) = current.next {
            current.next = swap_pairs(next.next);
            next.next = Some(current);
            return Some(next);
        } else {
            return Some(current);
        }
    }
    return None;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = swap_pairs(ListNode::from(vec![1, 2, 3, 4]));
        assert_eq!(result, ListNode::from(vec![2, 1, 4, 3]));
    }

    #[test]
    fn example_two() {
        let result = swap_pairs(ListNode::from(vec![]));
        assert_eq!(result, ListNode::from(vec![]));
    }

    #[test]
    fn example_three() {
        let result = swap_pairs(ListNode::from(vec![1]));
        assert_eq!(result, ListNode::from(vec![1]));
    }
}
