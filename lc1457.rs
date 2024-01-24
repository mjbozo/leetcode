// LeetCode problem 1457: Pseudo-Palindromic Paths in a Binary Tree
// https://leetcode.com/problems/pseudo-palindromic-paths-in-a-binary-tree/description/

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
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
    let left_left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let left_right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let mut left = TreeNode::new(3);
    left.left = left_left;
    left.right = left_right;
    let right_right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let mut right = TreeNode::new(1);
    right.right = right_right;
    let mut root = TreeNode::new(2);
    root.left = Some(Rc::new(RefCell::new(left)));
    root.right = Some(Rc::new(RefCell::new(right)));
    let arg = Some(Rc::new(RefCell::new(root)));
    let result = pseudo_palindromic_paths(arg);
    println!("Result = {result}");
}

fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut path = HashSet::new();
    let result = traverse(root.unwrap(), &mut path);
    return result;
}

fn traverse(node: Rc<RefCell<TreeNode>>, path: &mut HashSet<i32>) -> i32 {
    let inner = node.borrow();
    let mut total = 0;

    if !path.insert(inner.val) {
        path.remove(&inner.val);
    }
    
    match (&inner.left, &inner.right) {
        (None, None) => {
            let result = if path.len() <= 1 { 1 } else { 0 };
            if !path.remove(&inner.val) {
                path.insert(inner.val);
            }
            return result;
        },
        _ => {
            if let Some(left) = &inner.left {
                total += traverse(left.clone(), path);
            }

            if let Some(right) = &inner.right {
                total += traverse(right.clone(), path);
            }
        }
    }

    if !path.remove(&inner.val) {
        path.insert(inner.val);
    }
    return total;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let left_left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let left_right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let mut left = TreeNode::new(3);
        left.left = left_left;
        left.right = left_right;

        let right_right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let mut right = TreeNode::new(1);
        right.right = right_right;

        let mut root = TreeNode::new(2);
        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));

        let arg = Some(Rc::new(RefCell::new(root)));
        let result = pseudo_palindromic_paths(arg);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_two() {
        let left_right_right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let mut left_right = TreeNode::new(3);
        left_right.right = left_right_right;
        
        let left_left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let mut left = TreeNode::new(1);
        left.left = left_left;
        left.right = Some(Rc::new(RefCell::new(left_right)));

        let right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let mut root = TreeNode::new(2);
        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = right;

        let arg = Some(Rc::new(RefCell::new(root)));
        let result = pseudo_palindromic_paths(arg);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_three() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let result = pseudo_palindromic_paths(root);
        assert_eq!(result, 1);
    }
}
