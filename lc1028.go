// LeetCode problem LC1028: Recover a Tree From Preorder Traversal
// https://leetcode.com/problems/recover-a-tree-from-preorder-traversal/description/

package main

import (
	"fmt"
	"strconv"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

type DepthValue struct {
	Depth int
	Value int
}

func main() {
	result := recoverFromPreorder("1-2--3--4-5--6--7")
	fmt.Printf("%s\n", result)
}

func recoverFromPreorder(traversal string) *TreeNode {
	depthValues := make([]DepthValue, 0)

	currentValue := ""
	currentDepth := 0
	for _, c := range traversal {
		if c == '-' {
			if currentValue != "" {
				value, _ := strconv.Atoi(currentValue)
				depthValues = append(depthValues, DepthValue{Depth: currentDepth, Value: value})
				currentDepth = 0
				currentValue = ""
			}

			currentDepth += 1
		} else {
			currentValue += string(c)
		}
	}

	if currentValue != "" {
		value, _ := strconv.Atoi(currentValue)
		depthValues = append(depthValues, DepthValue{Depth: currentDepth, Value: value})
		currentDepth = 0
		currentValue = ""
	}

	root := &TreeNode{}
	PopulateTree(root, 0, &depthValues)

	return root
}

func PopulateTree(node *TreeNode, currentDepth int, depthValues *[]DepthValue) {
	if len(*depthValues) == 0 || (*depthValues)[0].Depth < currentDepth {
		return
	}

	currentDepthValue := (*depthValues)[0]
	*depthValues = (*depthValues)[1:]

	if currentDepthValue.Depth == currentDepth {
		node.Val = currentDepthValue.Value
	}

	if node.Left == nil && len(*depthValues) > 0 && (*depthValues)[0].Depth == currentDepth+1 {
		left := &TreeNode{}
		node.Left = left
		PopulateTree(left, currentDepth+1, depthValues)
	}

	if node.Right == nil && len(*depthValues) > 0 && (*depthValues)[0].Depth == currentDepth+1 {
		right := &TreeNode{}
		node.Right = right
		PopulateTree(right, currentDepth+1, depthValues)
	}
}
