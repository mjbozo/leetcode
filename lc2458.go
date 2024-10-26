// LeetCode problem 2458: Height of Binary Tree After Subtree Removal Queries
// https://leetcode.com/problems/height-of-binary-tree-after-subtree-removal-queries/description/

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

func (n *TreeNode) String(depth int) string {
	if n == nil {
		return "nil"
	}

	var pad string
	for range depth {
		pad += "  "
	}
	return fmt.Sprintf("%sTreeNode{\n%sVal: %d,\n%sLeft: %s,%s\nRight: %s\n%s}", pad, pad, n.Val, pad, n.Left.String(depth+1), pad, n.Right.String(depth+1), pad)
}

func main() {
	tree1 := &TreeNode{
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

	result := treeQueries(tree1, []int{3, 2, 4, 8})
	fmt.Printf("Result = %v\n", result)
}

func treeQueries(root *TreeNode, queries []int) []int {
	heightsAfterRemoval := make([]int, 0)

	heights := make(map[int]int)
	findHeight(root, &heights)

	precomps := make(map[int]int)
	calculateRows(root, &precomps, &heights)

	for _, query := range queries {
		heightsAfterRemoval = append(heightsAfterRemoval, precomps[query])
	}

	return heightsAfterRemoval
}

func findHeight(root *TreeNode, heights *map[int]int) int {
	if root == nil {
		return -1
	}

	height := max(findHeight(root.Left, heights), findHeight(root.Right, heights)) + 1
	(*heights)[root.Val] = height
	return height
}

// shame below
func calculateRows(root *TreeNode, precomps *map[int]int, heights *map[int]int) {
	// store maximum height as all non-longest path nodes will use this value
	tallest := (*heights)[root.Val]

	// create fifo queue for BFS traversal, inserting first 2 child nodes
	queue := make([]*TreeNode, 0)
	if root.Left != nil {
		queue = append(queue, root.Left)
	}
	if root.Right != nil {
		queue = append(queue, root.Right)
	}

	// using node with value -1 to signify end of row
	queue = append(queue, &TreeNode{Val: -1})

	// keep track of depth to know what the max would be after removing nodes
	depth := 1

	// begin BFS
	for len(queue) > 0 {
		// row is like a buffer to process each row of the tree at a time
		row := make([]*TreeNode, 0)
		current := queue[0]
		queue = queue[1:]

		// continue until end of row reached
		for current.Val != -1 {
			row = append(row, current)
			current = queue[0]
			queue = queue[1:]
		}

		// append all the children of current row to the queue
		for _, t := range row {
			if t.Left != nil {
				queue = append(queue, t.Left)
			}
			if t.Right != nil {
				queue = append(queue, t.Right)
			}
		}

		// only append EOR node if current row has values, otherwise we get locked
		if len(row) > 0 {
			queue = append(queue, &TreeNode{Val: -1})
		}

		// sort the row nodes descending by subtree height
		slices.SortFunc(row, func(a, b *TreeNode) int {
			return (*heights)[b.Val] - (*heights)[a.Val]
		})

		// precompute the values for each node in the row
		// if its the first (deepest subtree) then its value will be current
		// depth + depth of next deepest subtree
		// if only one node in row, then it is the deepest node, so its
		// value will be depth - 1
		for i := range len(row) {
			if i == 0 {
				if len(row) >= 2 {
					(*precomps)[row[0].Val] = depth + (*heights)[row[1].Val]
				} else {
					(*precomps)[row[0].Val] = depth - 1
				}
			} else {
				(*precomps)[row[i].Val] = tallest
			}
		}

		// increment depth for next row processing
		depth++
	}
}
