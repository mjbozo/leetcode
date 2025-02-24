// LeetCode problem 0889: Construct Binary Tree from Preorder and Postorder Traversal
// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-postorder-traversal/description/

package main

import (
	"fmt"
	"strings"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func (n *TreeNode) String(depth int) string {
	if n == nil {
		return "nil"
	}

	pad := strings.Repeat("  ", depth+1)
	var output = fmt.Sprintf("TreeNode[\n%sVal: %d\n%sLeft: %s\n%sRight: %s\n%s]", pad, n.Val, pad, n.Left.String(depth+1), pad, n.Right.String(depth+1), pad[2:])
	return output
}

func main() {
	result := constructFromPrePost([]int{1, 2, 4, 5, 3, 6, 7}, []int{4, 5, 2, 6, 7, 3, 1})
	fmt.Printf("Result = %s\n", result.String(0))
	result = constructFromPrePost([]int{1}, []int{1})
	fmt.Printf("Result = %s\n", result.String(0))
}

func constructFromPrePost(preorder []int, postorder []int) *TreeNode {
	order := make([]int, len(postorder)+1)
	for i, n := range postorder {
		order[n] = i
	}

	root := &TreeNode{Val: preorder[0]}
	preorder = preorder[1:]
	constructTree(root, &preorder, &order)
	return root
}

func constructTree(node *TreeNode, preorder *[]int, order *[]int) {
	if len(*preorder) == 0 {
		return
	}

	if (*order)[node.Val] > (*order)[(*preorder)[0]] && node.Left == nil {
		child := &TreeNode{Val: (*preorder)[0]}
		*preorder = (*preorder)[1:]
		node.Left = child
		constructTree(node.Left, preorder, order)
	}

	if len(*preorder) == 0 {
		return
	}

	if (*order)[node.Val] > (*order)[(*preorder)[0]] && node.Right == nil {
		child := &TreeNode{Val: (*preorder)[0]}
		*preorder = (*preorder)[1:]
		node.Right = child
		constructTree(node.Right, preorder, order)
	}
}
