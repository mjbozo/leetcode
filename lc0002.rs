// LeetCode problem 0002: Add Two Numbers
// https://leetcode.com/problems/add-two-numbers/description/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn main() {
    let tail1 = ListNode::new(3);
    let mut mid1 = ListNode::new(4);
    let mut head1 = ListNode::new(2);
    mid1.next = Some(Box::new(tail1));
    head1.next = Some(Box::new(mid1));
    
    let tail2 = ListNode::new(4);
    let mut mid2 = ListNode::new(6);
    let mut head2 = ListNode::new(5);
    mid2.next = Some(Box::new(tail2));
    head2.next = Some(Box::new(mid2));
    
    let result = add_two_numbers(Some(Box::new(head1)), Some(Box::new(head2)));
    println!("Result = {result:?}");
}

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if l1.is_none() && l2.is_none() {
        return None;
    }

    let l1_value = l1.as_ref().unwrap_or(&Box::new(ListNode::new(0))).val;
    let l2_value = l2.as_ref().unwrap_or(&Box::new(ListNode::new(0))).val;

    let mut current;
    if l1_value + l2_value >= 10 {
        let new_next = add_two_numbers(l1.clone().unwrap().next, Some(Box::new(ListNode::new(1))));
        let remainder = (l1_value + l2_value) % 10;
        current = ListNode::new(remainder);
        current.next = add_two_numbers(new_next, l2.unwrap().next);
    } else {
        current = ListNode::new(l1_value + l2_value);
        current.next = add_two_numbers(l1.unwrap_or(Box::new(ListNode::new(0))).next, l2.unwrap_or(Box::new(ListNode::new(0))).next);
    }

    return Some(Box::new(current));
}


fn build_list(nums: Vec<usize>) -> ListNode {
    let mut next: Option<Box<ListNode>> = None;
    let mut node = ListNode::new(0);
    for &num in nums.iter() {
        node = ListNode::new(num as i32);
        if let Some(n) = next {
            node.next = Some(n);
        }
        next = Some(Box::new(node.clone()));
    }
    return node;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let head1 = build_list(vec![2, 4, 3]);
        let head2 = build_list(vec![5, 6, 4]);
        let result = add_two_numbers(Some(Box::new(head1)), Some(Box::new(head2)));
        let res_head = build_list(vec![8, 0, 7]);
        assert_eq!(result, Some(Box::new(res_head)));
    }

    #[test]
    fn example_two() {
        let node1 = ListNode::new(0);
        let node2 = ListNode::new(0);
        let result = add_two_numbers(Some(Box::new(node1)), Some(Box::new(node2)));
        assert_eq!(result, Some(Box::new(ListNode::new(0))));
    }

    #[test]
    fn example_three() {
        let node1 = build_list(vec![9,9,9,9,9,9,9]);
        let node2 = build_list(vec![9,9,9,9]);
        let result = add_two_numbers(Some(Box::new(node1)), Some(Box::new(node2)));
        let res_head = build_list(vec![1, 0, 0, 0, 9, 9, 9, 8]);
        assert_eq!(result, Some(Box::new(res_head)));
    }
}
