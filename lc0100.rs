// LeetCode problem 0100: Same Tree
// https://leetcode.com/problems/same-tree/description/

use std::cell::RefCell;
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
    let mut left_root = TreeNode::new(1);
    left_root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));

    let mut right_root = TreeNode::new(1);
    right_root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));

    let left = Rc::new(RefCell::new(left_root));
    let right = Rc::new(RefCell::new(right_root));
    let result = is_same_tree(Some(left), Some(right));
    println!("Result = {result}");
}

fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    return p == q;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let mut left_root = TreeNode::new(1);
        left_root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        left_root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));

        let mut right_root = TreeNode::new(1);
        right_root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        right_root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));

        let left = Some(Rc::new(RefCell::new(left_root)));
        let right = Some(Rc::new(RefCell::new(right_root)));
        let result = is_same_tree(left, right);
        assert!(result);
    }

    #[test]
    fn example_two() {
        let mut left_root = TreeNode::new(1);
        left_root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));

        let mut right_root = TreeNode::new(1);
        right_root.right = Some(Rc::new(RefCell::new(TreeNode::new(2))));

        let left = Rc::new(RefCell::new(left_root));
        let right = Rc::new(RefCell::new(right_root));
        let result = is_same_tree(Some(left), Some(right));
        assert!(!result);
    }

    #[test]
    fn example_three() {
        let mut left_root = TreeNode::new(1);
        left_root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        left_root.right = Some(Rc::new(RefCell::new(TreeNode::new(1))));

        let mut right_root = TreeNode::new(1);
        right_root.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        right_root.right = Some(Rc::new(RefCell::new(TreeNode::new(2))));

        let left = Some(Rc::new(RefCell::new(left_root)));
        let right = Some(Rc::new(RefCell::new(right_root)));
        let result = is_same_tree(left, right);
        assert!(!result);
    }
}
