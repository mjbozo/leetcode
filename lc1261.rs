// LeetCode problem 1261: Find Elements in a Contaminated Binary Tree
// https://leetcode.com/problems/find-elements-in-a-contaminated-binary-tree/description/

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        return TreeNode {
            val,
            left: None,
            right: None,
        };
    }
}

struct FindElements {
    tree_nums: HashSet<i32>,
}

impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut find_elements = FindElements {
            tree_nums: HashSet::new(),
        };

        if root.is_some() {
            find_elements.tree_nums.insert(0);
        }

        find_elements.dfs(&root, 0);

        return find_elements;
    }

    fn dfs(&mut self, node: &Option<Rc<RefCell<TreeNode>>>, current_val: i32) {
        if let Some(x) = node {
            let parent = x.borrow();
            let left = &parent.left;
            if left.is_some() {
                let left_val = current_val * 2 + 1;
                self.tree_nums.insert(left_val);
                self.dfs(left, left_val);
            }

            let right = &parent.right;
            if right.is_some() {
                let right_val = current_val * 2 + 2;
                self.tree_nums.insert(right_val);
                self.dfs(right, right_val);
            }
        }
    }

    fn find(&self, target: i32) -> bool {
        return self.tree_nums.contains(&target);
    }
}

fn main() {
    let mut root = TreeNode::new(-1);
    let right = Some(Rc::new(RefCell::new(TreeNode::new(-1))));
    root.right = right;

    let fe = FindElements::new(Some(Rc::new(RefCell::new(root))));
    println!("{}", fe.find(1));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let mut root = TreeNode::new(-1);
        let right = Some(Rc::new(RefCell::new(TreeNode::new(-1))));
        root.right = right;

        let fe = FindElements::new(Some(Rc::new(RefCell::new(root))));
        assert!(!fe.find(1));
        assert!(fe.find(2));
    }

    #[test]
    fn example_two() {
        let mut root = TreeNode::new(-1);
        let mut left = TreeNode::new(-1);
        let left_left = TreeNode::new(-1);
        let left_right = TreeNode::new(-1);
        let right = TreeNode::new(-1);
        left.left = Some(Rc::new(RefCell::new(left_left)));
        left.right = Some(Rc::new(RefCell::new(left_right)));
        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));

        let fe = FindElements::new(Some(Rc::new(RefCell::new(root))));
        assert!(fe.find(1));
        assert!(fe.find(3));
        assert!(!fe.find(5));
    }

    #[test]
    fn example_three() {
        let mut root = TreeNode::new(-1);
        let mut right = TreeNode::new(-1);
        let mut right_left = TreeNode::new(-1);
        let right_left_left = TreeNode::new(-1);
        right_left.left = Some(Rc::new(RefCell::new(right_left_left)));
        right.left = Some(Rc::new(RefCell::new(right_left)));
        root.right = Some(Rc::new(RefCell::new(right)));

        let fe = FindElements::new(Some(Rc::new(RefCell::new(root))));
        assert!(fe.find(2));
        assert!(!fe.find(3));
        assert!(!fe.find(4));
        assert!(fe.find(5));
    }
}
