// LeetCode problem 0951: Flip Equivalent Binary Trees
// https://leetcode.com/problems/flip-equivalent-binary-trees/description/

package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func main() {
	tree1 := &TreeNode{
		Val: 1,
		Left: &TreeNode{
			Val: 2,
			Left: &TreeNode{
				Val: 4,
			},
			Right: &TreeNode{
				Val: 5,
				Left: &TreeNode{
					Val: 7,
				},
				Right: &TreeNode{
					Val: 8,
				},
			},
		},
		Right: &TreeNode{
			Val: 3,
			Left: &TreeNode{
				Val: 6,
			},
		},
	}

	tree2 := &TreeNode{
		Val: 1,
		Left: &TreeNode{
			Val: 3,
			Right: &TreeNode{
				Val: 6,
			},
		},
		Right: &TreeNode{
			Val: 2,
			Left: &TreeNode{
				Val: 4,
			},
			Right: &TreeNode{
				Val: 5,
				Left: &TreeNode{
					Val: 8,
				},
				Right: &TreeNode{
					Val: 7,
				},
			},
		},
	}

	result := flipEquiv(tree2, tree1)
	fmt.Printf("Result = %t\n", result)
}

func flipEquiv(root1 *TreeNode, root2 *TreeNode) bool {
	if root1 == nil && root2 == nil {
		return true
	}

	if root1 == nil || root2 == nil {
		return false
	}

	if root1.Val != root2.Val {
		return false
	}

	leftMatches := flipEquiv(root1.Left, root2.Left) || flipEquiv(root1.Left, root2.Right)
	rightMatches := flipEquiv(root1.Right, root2.Left) || flipEquiv(root1.Right, root2.Right)

	return leftMatches && rightMatches
}
