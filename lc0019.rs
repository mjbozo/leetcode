// LeetCode problem 0019: Remove Nth Node From End of List
// https://leetcode.com/problems/remove-nth-node-from-end-of-list/description/

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
    let result = remove_nth_from_end(ListNode::build_list(vec![1,2,3,4,5]), 2);
    println!("Result = {result:?}");
}

fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut len = 0;
    let mut current = head.as_ref();
    while let Some(n) = current {
        len += 1;
        current = n.next.as_ref();
    }

    println!("Len = {len}");
    println!("Head is {head:?}");
    let start = head.as_ref();
    current = head.as_ref();
    for i in 0..len {
        if i == len - n {
            current.as_mut().unwrap().next = Some(current.unwrap().next.clone().unwrap().next.unwrap());
            break;
        }
        current = current.unwrap().next.as_ref();
    }

    return start.cloned();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = remove_nth_from_end(ListNode::build_list(vec![1,2,3,4,5]), 2);
        assert_eq!(result, ListNode::build_list(vec![1,2,3,5]));
    }

    #[test]
    fn example_two() {
        let result = remove_nth_from_end(ListNode::build_list(vec![1]), 1);
        assert_eq!(result, ListNode::build_list(vec![]));
    }
    
    #[test]
    fn example_one() {
        let result = remove_nth_from_end(ListNode::build_list(vec![1,2]), 1);
        assert_eq!(result, ListNode::build_list(vec![1]));
    }
}
