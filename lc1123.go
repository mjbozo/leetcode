// LeetCode problem 1123: Lowest Common Ancestor of Deepest Leaves
// https://leetcode.com/problems/lowest-common-ancestor-of-deepest-leaves/description/

package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func (n TreeNode) String() string {
	return fmt.Sprintf("TreeNode{Val: %d}", n.Val)
}

type NodeDepth struct {
	node  *TreeNode
	depth int
}

func main() {
	root := &TreeNode{Val: 3}

	left := &TreeNode{Val: 5}
	leftLeft := &TreeNode{Val: 6}
	left.Left = leftLeft

	leftRight := &TreeNode{Val: 2}
	leftRightLeft := &TreeNode{Val: 7}
	leftRightRight := &TreeNode{Val: 4}
	leftRight.Left = leftRightLeft
	leftRight.Right = leftRightRight
	left.Right = leftRight

	right := &TreeNode{Val: 1}
	rightLeft := &TreeNode{Val: 0}
	right.Left = rightLeft

	rightRight := &TreeNode{Val: 8}
	right.Right = rightRight

	root.Left = left
	root.Right = right

	result := lcaDeepestLeaves(root)
	fmt.Printf("Result = %s\n", result)
}

func lcaDeepestLeaves(root *TreeNode) *TreeNode {
	deepest, numDeepest := findDeepest(root)

	deepestNodes := make(map[*TreeNode]int)
	allocateDeepest(root, deepestNodes, deepest, 0)

	deepestSubroot := findDeepestSubroot(root, numDeepest, deepestNodes)

	return deepestSubroot
}

func findDeepestSubroot(node *TreeNode, numDeepest int, nodeMap map[*TreeNode]int) *TreeNode {
	if node.Left != nil && nodeMap[node.Left] == numDeepest {
		return findDeepestSubroot(node.Left, numDeepest, nodeMap)
	}

	if node.Right != nil && nodeMap[node.Right] == numDeepest {
		return findDeepestSubroot(node.Right, numDeepest, nodeMap)
	}

	return node
}

func allocateDeepest(node *TreeNode, nodeMap map[*TreeNode]int, deepest, currentDepth int) int {
	if node == nil {
		return 0
	}

	if currentDepth == deepest {
		nodeMap[node] = 1
		return 1
	}

	left := allocateDeepest(node.Left, nodeMap, deepest, currentDepth+1)
	right := allocateDeepest(node.Right, nodeMap, deepest, currentDepth+1)

	nodeMap[node] = left + right
	return nodeMap[node]
}

func findDeepest(root *TreeNode) (int, int) {
	deepest := 0
	numDeepest := 0
	nodes := make(chan NodeDepth, 1000)
	nodes <- NodeDepth{node: root, depth: 0}

	for len(nodes) > 0 {
		current := <-nodes

		if current.depth > deepest {
			deepest = current.depth
			numDeepest = 1
		} else if current.depth == deepest {
			numDeepest++
		}

		if current.node.Left != nil {
			nodes <- NodeDepth{node: current.node.Left, depth: current.depth + 1}
		}
		if current.node.Right != nil {
			nodes <- NodeDepth{node: current.node.Right, depth: current.depth + 1}
		}
	}

	return deepest, numDeepest
}
