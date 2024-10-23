// LeetCode problem 2641: Cousins in Binary Tree II
// https://leetcode.com/problems/cousins-in-binary-tree-ii/description/

package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func (t *TreeNode) String() string {
	if t == nil {
		return "nil"
	}

	return fmt.Sprintf("Node { Val: %d, Left: %s, Right: %s }", t.Val, t.Left.String(), t.Right.String())
}

type DepthValue struct {
	Depth int
	Node  *TreeNode
}

func main() {
	tree := &TreeNode{
		Val: 5,
		Left: &TreeNode{
			Val: 4,
			Left: &TreeNode{
				Val: 1,
			},
			Right: &TreeNode{
				Val: 10,
			},
		},
		Right: &TreeNode{
			Val: 9,
			Right: &TreeNode{
				Val: 7,
			},
		},
	}

	result := replaceValueInTree(tree)
	fmt.Printf("Result = %q\n", result.String())
}

func replaceValueInTree(root *TreeNode) *TreeNode {
	queue := make([]DepthValue, 0)
	queue = append(queue, DepthValue{Depth: 0, Node: root})

	sums := make([]int, 0)

	for len(queue) > 0 {
		current := queue[0]
		queue = queue[1:]

		if len(sums) <= current.Depth {
			sums = append(sums, current.Node.Val)
		} else {
			sums[current.Depth] += current.Node.Val
		}

		if current.Node.Left != nil {
			queue = append(queue, DepthValue{
				Depth: current.Depth + 1,
				Node:  current.Node.Left,
			})
		}

		if current.Node.Right != nil {
			queue = append(queue, DepthValue{
				Depth: current.Depth + 1,
				Node:  current.Node.Right,
			})
		}
	}

	fmt.Printf("Sums = %v\n", sums)

	propagateValues(root, 0, &sums)

	root.Val = 0
	return root
}

func propagateValues(node *TreeNode, depth int, sums *[]int) {
	if depth >= len(*sums)-1 {
		return
	}

	nextDepthSum := (*sums)[depth+1]
	childrenSum := 0

	if node.Left != nil {
		childrenSum += node.Left.Val
	}

	if node.Right != nil {
		childrenSum += node.Right.Val
	}

	newChildValue := nextDepthSum - childrenSum

	if node.Left != nil {
		node.Left.Val = newChildValue
		propagateValues(node.Left, depth+1, sums)
	}

	if node.Right != nil {
		node.Right.Val = newChildValue
		propagateValues(node.Right, depth+1, sums)
	}
}
