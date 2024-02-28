// LeetCode problem 0513: Find Bottom Left Tree Value
// https://leetcode.com/problems/find-bottom-left-tree-value/description/

use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

#[derive(PartialEq, Eq)]
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
    let mut root = TreeNode::new(2);
    let left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.left = left;
    root.right = right;
    let result = find_bottom_left_value(Some(Rc::new(RefCell::new(root))));
    println!("Result = {result}");
}

fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    return deepest_left(root).0;
}

fn deepest_left(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) /* (value, depth) */ {
    let node = node.as_ref().unwrap();

    let (mut left_value, mut left_depth) = (node.borrow().val, 0);
    if let Some(left) = &node.borrow().left {
        (left_value, left_depth) = deepest_left(Some(left.clone()));
        left_depth += 1;
    }
    let (mut right_value, mut right_depth) = (node.borrow().val, 0);
    if let Some(right) = &node.borrow().right {
        (right_value, right_depth) = deepest_left(Some(right.clone()));
        right_depth += 1;
    }

    return match left_depth.cmp(&right_depth) {
        Ordering::Less => (right_value, right_depth),
        _ => (left_value, left_depth)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let mut root = TreeNode::new(2);
        let left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        root.left = left;
        root.right = right;
        let result = find_bottom_left_value(Some(Rc::new(RefCell::new(root))));
        assert_eq!(result, 1);
    }

    #[test]
    fn example_two() {
        let mut root = TreeNode::new(1);
        let mut l = TreeNode::new(2);
        let ll = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        l.left = ll;
        root.left = Some(Rc::new(RefCell::new(l)));

        let mut r = TreeNode::new(3);
        let mut rl = TreeNode::new(5);
        let rll = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        rl.left = rll;
        let rr = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        r.left = Some(Rc::new(RefCell::new(rl)));
        r.right = rr;
        
        root.right = Some(Rc::new(RefCell::new(r)));
        let result = find_bottom_left_value(Some(Rc::new(RefCell::new(root))));
        assert_eq!(result, 7);
    }
}
