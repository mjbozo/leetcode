// LeetCode problem 3373: Maximize the Number of Target Nodes After Connecting Trees II
// https://leetcode.com/problems/maximize-the-number-of-target-nodes-after-connecting-trees-ii/description/

package main

import (
	"fmt"
)

func main() {
	edges1 := [][]int{[]int{0, 1}, []int{0, 2}, []int{2, 3}, []int{2, 4}}
	edges2 := [][]int{[]int{0, 1}, []int{0, 2}, []int{0, 3}, []int{2, 7}, []int{1, 4}, []int{4, 5}, []int{4, 6}}
	result := maxTargetNodes(edges1, edges2)
	fmt.Printf("Result = %v\n", result)
}

func maxTargetNodes(edges1 [][]int, edges2 [][]int) []int {
	tree1 := buildAdjacencyList(edges1)
	tree2 := buildAdjacencyList(edges2)

	lookup := make(map[int]int)
	lookup[-1] = -1
	even := 0
	odd := 0

	dfs(0, tree1, -1, lookup, &even, &odd)

	maximums := make([]int, len(edges1)+1)
	for i := range tree1 {
		if lookup[i]%2 == 0 {
			maximums[i] = even
		} else {
			maximums[i] = odd
		}
	}

	lookup = make(map[int]int)
	lookup[-1] = -1
	even = 0
	odd = 0

	dfs(0, tree2, -1, lookup, &even, &odd)
	tree2Value := max(even, odd)

	for i := range maximums {
		maximums[i] += tree2Value
	}

	return maximums
}

func dfs(current int, tree [][]int, parent int, lookup map[int]int, even *int, odd *int) {
	lookup[current] = lookup[parent] + 1
	if lookup[current]%2 == 0 {
		*even++
	} else {
		*odd++
	}

	for _, next := range tree[current] {
		if next == parent {
			continue
		}

		dfs(next, tree, current, lookup, even, odd)
	}
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
