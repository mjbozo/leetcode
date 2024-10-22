// LeetCode problem 2583: Kth Largest Sum in a Binary Tree
// https://leetcode.com/problems/kth-largest-sum-in-a-binary-tree/description/

package main

import (
	"fmt"
	"slices"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

type DepthValue struct {
	Depth int
	Node  *TreeNode
}

func main() {
	tree := &TreeNode{
		Val: 5,
		Left: &TreeNode{
			Val: 8,
			Left: &TreeNode{
				Val: 2,
				Left: &TreeNode{
					Val: 4,
				},
				Right: &TreeNode{
					Val: 6,
				},
			},
			Right: &TreeNode{
				Val: 1,
			},
		},
		Right: &TreeNode{
			Val: 9,
			Left: &TreeNode{
				Val: 3,
			},
			Right: &TreeNode{
				Val: 7,
			},
		},
	}

	result := kthLargestLevelSum(tree, 2)
	fmt.Printf("Result = %v\n", result)
}

func kthLargestLevelSum(root *TreeNode, k int) int64 {
	queue := make([]DepthValue, 0)
	queue = append(queue, DepthValue{Depth: 0, Node: root})

	sums := make([]int64, 0)

	for len(queue) > 0 {
		current := queue[0]
		queue = queue[1:]

		if len(sums) <= current.Depth {
			sums = append(sums, int64(current.Node.Val))
		} else {
			sums[current.Depth] += int64(current.Node.Val)
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

	slices.Sort(sums)
	slices.Reverse(sums)

	if len(sums) < k {
		return -1
	}

	return sums[k-1]
}
