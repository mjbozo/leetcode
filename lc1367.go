// LeetCode problem 1367: Linked List in Binary Tree
// https://leetcode.com/problems/linked-list-in-binary-tree/description/

package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func build(vals []int) *ListNode {
	if len(vals) == 0 {
		return nil
	}

	node := ListNode{Val: vals[0]}
	node.Next = build(vals[1:])
	return &node
}

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func main() {
	// would love to find a way to avoid this
	tree := &TreeNode{
		Val: 1,
		Left: &TreeNode{
			Val:  4,
			Left: nil,
			Right: &TreeNode{
				Val: 2,
				Left: &TreeNode{
					Val:   1,
					Left:  nil,
					Right: nil,
				},
				Right: nil,
			},
		},
		Right: &TreeNode{
			Val: 4,
			Left: &TreeNode{
				Val: 2,
				Left: &TreeNode{
					Val:   6,
					Left:  nil,
					Right: nil,
				},
				Right: &TreeNode{
					Val: 8,
					Left: &TreeNode{
						Val:   1,
						Left:  nil,
						Right: nil,
					},
					Right: &TreeNode{
						Val:   3,
						Left:  nil,
						Right: nil,
					},
				},
			},
		},
	}

	result := isSubPath(build([]int{4, 2, 8}), tree)
	fmt.Printf("Result = %t\n", result)
}

func isSubPath(head *ListNode, root *TreeNode) bool {
	if root == nil {
		return false
	}

	if traverse(head, root) {
		return true
	}

	return isSubPath(head, root.Left) || isSubPath(head, root.Right)
}

func traverse(head *ListNode, node *TreeNode) bool {
	if head == nil {
		return true
	}

	if node == nil {
		return false
	}

	if head.Val != node.Val {
		return false
	}

	return traverse(head.Next, node.Left) || traverse(head.Next, node.Right)
}
