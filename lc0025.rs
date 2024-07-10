// LeetCode problem 0025: Reverse Nodes in k-Group
// https://leetcode.com/problems/reverse-nodes-in-k-group/description/

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
    let result = reverse_k_group(ListNode::from(vec![1,2,3,4,5]), 2);
    println!("Result = {result:?}");
}

fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if k == 1 {
        return head;
    }

    let mut stack = vec![];
    let mut current = head.as_ref();
    for _ in 0..k {
        if let Some(c) = current {
            stack.push(c);
            current = c.next;
        } else {
            return head;
        }
    }

    // current IS next here

    let new_head = stack.pop();
    let mut new_current = new_head;
    while let Some(node) = stack.pop() {
        if let Some(mut c) = new_current {
            c.next = Some(node);
            new_current = Some(node);
        }
    }

    if let Some(mut c) = new_current {
        c.next = reverse_k_group(current, k);
    }

    return new_head;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = reverse_k_group(ListNode::from(vec![1,2,3,4,5]), 2);
        assert_eq!(result, ListNode::from(vec![2,1,4,3,5]));
    }

    #[test]
    fn example_two() {
        let result = reverse_k_group(ListNode::from(vec![1,2,3,4,5]), 3);
        assert_eq!(result, ListNode::from(vec![3,2,1,4,5]));
    }
}
