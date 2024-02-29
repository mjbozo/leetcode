// LeetCode problem 1609: Even Odd Tree
// https://leetcode.com/problems/even-odd-tree/description/

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>
}

impl TreeNode {
    #[inline]
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}

fn main() {
    let lll = Some(Rc::new(RefCell::new(TreeNode::new(12))));
    let llr = Some(Rc::new(RefCell::new(TreeNode::new(8))));
    let ll = Some(Rc::new(RefCell::new(TreeNode { val: 3, left: lll, right: llr })));
    let l = Some(Rc::new(RefCell::new(TreeNode { val: 10, left: ll, right: None })));
    let rll = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    let rl = Some(Rc::new(RefCell::new(TreeNode { val: 7, left: rll, right: None })));
    let rrr = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let rr = Some(Rc::new(RefCell::new(TreeNode { val: 9, left: None, right: rrr })));
    let r = Some(Rc::new(RefCell::new(TreeNode { val: 4, left: rl, right: rr })));
    let root = Some(Rc::new(RefCell::new(TreeNode { val: 1, left: l, right: r })));
    let result = is_even_odd_tree(root);
    println!("Result = {result}");
}

fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut level_nodes: Vec<i32> = vec![];
    let mut current_level = 0;

    let mut open_set = VecDeque::new();
    open_set.push_back((root.unwrap(), current_level));
    while !open_set.is_empty() {
        let (node, level) = open_set.pop_front().unwrap();
        if level != current_level {
            let even_level = current_level % 2 == 0;
            let node_slice = level_nodes.as_slice();
            let all_even = node_slice.iter().all(|v| v % 2 == 0);
            let all_odd = node_slice.iter().all(|v| v % 2 == 1);
            let sorted_asc = level_nodes.as_slice().windows(2).all(|w| w[0] < w[1]);
            let sorted_desc = level_nodes.as_slice().windows(2).all(|w| w[0] > w[1]);

            if (even_level && (!all_odd || !sorted_asc)) || (!even_level && (!all_even || !sorted_desc)) {
                return false;
            }

            level_nodes.clear();
            current_level += 1;
        }

        level_nodes.push(node.borrow().val);

        if let Some(l) = &node.borrow().left {
            open_set.push_back((l.clone(), current_level + 1));
        };

        if let Some(r) = &node.borrow().right {
            open_set.push_back((r.clone(), current_level + 1));
        };
    }

    // check last level
    let even_level = current_level % 2 == 0;
    let node_slice = level_nodes.as_slice();
    let all_even = node_slice.iter().all(|v| v % 2 == 0);
    let all_odd = node_slice.iter().all(|v| v % 2 == 1);
    let sorted_asc = level_nodes.as_slice().windows(2).all(|w| w[0] < w[1]);
    let sorted_desc = level_nodes.as_slice().windows(2).all(|w| w[0] > w[1]);
    if (even_level && (!all_odd || !sorted_asc)) || (!even_level && (!all_even || !sorted_desc)) {
        return false;
    }

    return true;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let lll = Some(Rc::new(RefCell::new(TreeNode::new(12))));
        let llr = Some(Rc::new(RefCell::new(TreeNode::new(8))));
        let ll = Some(Rc::new(RefCell::new(TreeNode { val: 3, left: lll, right: llr })));
        let l = Some(Rc::new(RefCell::new(TreeNode { val: 10, left: ll, right: None })));
        let rll = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        let rl = Some(Rc::new(RefCell::new(TreeNode { val: 7, left: rll, right: None })));
        let rrr = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let rr = Some(Rc::new(RefCell::new(TreeNode { val: 9, left: None, right: rrr })));
        let r = Some(Rc::new(RefCell::new(TreeNode { val: 4, left: rl, right: rr })));
        let root = Some(Rc::new(RefCell::new(TreeNode { val: 1, left: l, right: r })));
        let result = is_even_odd_tree(root);
        assert!(result);
    }

    #[test]
    fn example_two() {
        let ll = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let lr = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let l = Some(Rc::new(RefCell::new(TreeNode { val: 4, left: ll, right: lr })));
        let rl = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let r = Some(Rc::new(RefCell::new(TreeNode { val: 2, left: rl, right: None })));
        let root = Some(Rc::new(RefCell::new(TreeNode { val: 5, left: l, right: r })));
        let result = is_even_odd_tree(root);
        assert!(!result);
    }

    #[test]
    fn example_three() {
        let ll = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let lr = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let l = Some(Rc::new(RefCell::new(TreeNode { val: 9, left: ll, right: lr })));
        let rl = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let r = Some(Rc::new(RefCell::new(TreeNode { val: 1, left: rl, right: None })));
        let root = Some(Rc::new(RefCell::new(TreeNode { val: 5, left: l, right: r })));
        let result = is_even_odd_tree(root);
        assert!(!result);
    }
}
