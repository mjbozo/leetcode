// LeetCode problem 0876: Middle of the Linked List
// https://leetcode.com/problems/middle-of-the-linked-list/description/

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

    fn build_list(vals: Vec<i32>) -> Option<Box<ListNode>> {
        if vals.len() == 0 {
            return None;
        }

        let mut node = ListNode::new(vals[0]);
        node.next = Self::build_list(vals[1..].to_vec());

        return Some(Box::new(node));
    }
}

fn main() {
    let result = middle_node(ListNode::build_list(vec![1,2,3,4,5]));
    println!("Result = {result:?}");
}

fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut slow = head.as_ref();
    let mut fast = head.as_ref();

    while fast.is_some() && fast.unwrap().next.is_some() {
        slow = slow.unwrap().next.as_ref();
        fast = fast.unwrap().next.as_ref().unwrap().next.as_ref();
    }

    return Some(slow.unwrap().clone());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = middle_node(ListNode::build_list(vec![1,2,3,4,5]));
        assert_eq!(result, ListNode::build_list(vec![3,4,5]));
    }

    #[test]
    fn example_two() {
        let result = middle_node(ListNode::build_list(vec![1,2,3,4,5,6]));
        assert_eq!(result, ListNode::build_list(vec![4,5,6]));
    }
}
