// LeetCode problem 0021: Merge Two Sorted Lists
// https://leetcode.com/problems/merge-two-sorted-lists/description/

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
    let list1 = build_list(vec![1, 2, 4]);
    let list2 = build_list(vec![1, 3, 4]);
    let result = merge_two_lists(list1, list2);
    println!("{result:?}");
}

fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut start = Some(Box::new(ListNode::new(0)));
    let mut current = &mut start;
    
    while list1.is_some() || list2.is_some() {
        match (&list1, &list2) {
            (Some(x), Some(y)) => {
                if x.val <= y.val {
                    current.as_mut().unwrap().next = list1.clone();
                    current = &mut current.as_mut().unwrap().next;
                    list1 = list1.unwrap().next;
                } else {
                    current.as_mut().unwrap().next = list2.clone();
                    current = &mut current.as_mut().unwrap().next;
                    list2 = list2.unwrap().next;
                }
            },
            (Some(_), None) => {
                    current.as_mut().unwrap().next = list1.clone();
                    current = &mut current.as_mut().unwrap().next;
                    list1 = list1.unwrap().next;
            },
            (None, Some(_)) => {
                    current.as_mut().unwrap().next = list2.clone();
                    current = &mut current.as_mut().unwrap().next;
                    list2 = list2.unwrap().next;
            },
            (None, None) => {
                break;
            }
        }
    }

    return start.unwrap().next;
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
        let result = merge_two_lists(build_list(vec![1, 2, 4]), build_list(vec![1, 3, 4]));
        assert_eq!(result, build_list(vec![1, 1, 2, 3, 4, 4]));
    }

    #[test]
    fn example_two() {
        let result = merge_two_lists(None, None);
        assert_eq!(result, None);
    }

    #[test]
    fn example_three() {
        let result = merge_two_lists(None, build_list(vec![0]));
        assert_eq!(result, build_list(vec![0]));
    }
}
