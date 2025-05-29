// LeetCode problem 3372: Maximize the Number of Target Nodes After Connecting Trees I
// https://leetcode.com/problems/maximize-the-number-of-target-nodes-after-connecting-trees-i/description/

package main

import (
	"fmt"
	"slices"
)

func main() {
	edges1 := [][]int{[]int{0, 1}, []int{0, 2}, []int{2, 3}, []int{2, 4}}
	edges2 := [][]int{[]int{0, 1}, []int{0, 2}, []int{0, 3}, []int{2, 7}, []int{1, 4}, []int{4, 5}, []int{4, 6}}
	result := maxTargetNodes(edges1, edges2, 2)
	fmt.Printf("Result = %v\n", result)
}

func maxTargetNodes(edges1 [][]int, edges2 [][]int, k int) []int {
	tree1 := buildAdjacencyList(edges1)
	tree2 := buildAdjacencyList(edges2)

	tree1Values := calculateNodeValues(tree1, k)
	tree2Values := calculateNodeValues(tree2, k-1)

	tree2Max := slices.Max(tree2Values)
	maximums := make([]int, 0)

	for _, val := range tree1Values {
		maximums = append(maximums, val+tree2Max)
	}

	return maximums
}

func buildAdjacencyList(edges [][]int) [][]int {
	n := len(edges) + 1
	tree := make([][]int, n)
	for _, edge := range edges {
		tree[edge[0]] = append(tree[edge[0]], edge[1])
		tree[edge[1]] = append(tree[edge[1]], edge[0])
	}
	return tree
}

func calculateNodeValues(tree [][]int, k int) []int {
	nodeValues := make([]int, len(tree))
	for n, _ := range tree {
		nodeValues[n] = dfs(n, -1, tree, k)
	}
	return nodeValues
}

func dfs(current int, cameFrom int, tree [][]int, k int) int {
	if k < 0 {
		return 0
	}

	total := 1
	for _, next := range tree[current] {
		if next == cameFrom {
			continue
		}

		total += dfs(next, current, tree, k-1)
	}
	return total
}
