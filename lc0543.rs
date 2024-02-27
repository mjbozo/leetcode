// LeetCode problem 0543: Diameter of Binary Tree
// https://leetcode.com/problems/diameter-of-binary-tree/description/

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
    let mut root = TreeNode::new(1);

    let mut left = TreeNode::new(2);
    let left_left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    let left_right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    left.left = left_left;
    left.right = left_right;

    let right = Some(Rc::new(RefCell::new(TreeNode::new(3))));

    root.left = Some(Rc::new(RefCell::new(left)));
    root.right = right;

    let result = diameter_of_binary_tree(Some(Rc::new(RefCell::new(root))));
    println!("Result = {result}");
}

fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let (_deepest, max) = max_depths(&root);
    return max;
}

fn max_depths(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) /* (deepest, max) */ {
    let node = node.as_ref().unwrap();

    let (mut left_deepest, mut left_max) = (0, 0);
    if let Some(left) = &node.borrow().left {
        (left_deepest, left_max) = max_depths(&Some(left.clone()));
    }

    let (mut right_deepest, mut right_max) = (0, 0);
    if let Some(right) = &node.borrow().right {
        (right_deepest, right_max) = max_depths(&Some(right.clone()));
    }

    let deepest = left_deepest.max(right_deepest) + 1;
    let max = left_max.max(right_max).max(left_deepest + right_deepest);

    return (deepest, max);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let mut root = TreeNode::new(1);

        let mut left = TreeNode::new(2);
        let left_left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let left_right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        left.left = left_left;
        left.right = left_right;

        let right = Some(Rc::new(RefCell::new(TreeNode::new(3))));

        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = right;

        let result = diameter_of_binary_tree(Some(Rc::new(RefCell::new(root))));
        assert_eq!(result, 3);
    }

    #[test]
    fn example_two() {
        let mut root = TreeNode::new(1);
        let left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        root.left = left;

        let result = diameter_of_binary_tree(Some(Rc::new(RefCell::new(root))));
        assert_eq!(result, 1);
    }
}
