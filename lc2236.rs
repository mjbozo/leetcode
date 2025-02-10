// LeetCode problem 2236: Root Equals Sum of Children
// https://leetcode.com/problems/root-equals-sum-of-children/description/

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

fn main() {
    let left = TreeNode {
        val: 4,
        left: None,
        right: None,
    };
    let right = TreeNode {
        val: 6,
        left: None,
        right: None,
    };
    let root = TreeNode {
        val: 10,
        left: Some(Rc::new(RefCell::new(left))),
        right: Some(Rc::new(RefCell::new(right))),
    };

    let result = check_tree(Some(Rc::new(RefCell::new(root))));
    println!("Result = {result}");
}

fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let root_ref = root.as_ref().unwrap().borrow();
    let root_val = root_ref.val;
    let left = root_ref.left.as_ref().unwrap().borrow().val;
    let right = root_ref.right.as_ref().unwrap().borrow().val;
    return root_val == left + right;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let left = TreeNode {
            val: 4,
            left: None,
            right: None,
        };
        let right = TreeNode {
            val: 6,
            left: None,
            right: None,
        };
        let root = TreeNode {
            val: 10,
            left: Some(Rc::new(RefCell::new(left))),
            right: Some(Rc::new(RefCell::new(right))),
        };
        assert!(check_tree(Some(Rc::new(RefCell::new(root)))));
    }

    #[test]
    fn example_two() {
        let left = TreeNode {
            val: 3,
            left: None,
            right: None,
        };
        let right = TreeNode {
            val: 1,
            left: None,
            right: None,
        };
        let root = TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(left))),
            right: Some(Rc::new(RefCell::new(right))),
        };
        assert!(!check_tree(Some(Rc::new(RefCell::new(root)))));
    }
}
