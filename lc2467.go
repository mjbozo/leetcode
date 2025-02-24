// LeetCode problem 2467: Most Profitable Path in a Tree
// https://leetcode.com/problems/most-profitable-path-in-a-tree/description/

package main

import (
	"fmt"
	"math"
)

type Node struct {
	index      int
	cost       int
	neighbours []*Node
}

func main() {
	result := mostProfitablePath([][]int{{0, 1}, {1, 2}, {1, 3}, {3, 4}}, 3, []int{-2, 4, 2, -4, 6})
	fmt.Printf("Result = %d\n", result)

	result = mostProfitablePath([][]int{{0, 1}}, 1, []int{-7280, 2350})
	fmt.Printf("Result = %d\n", result)
}

func mostProfitablePath(edges [][]int, bob int, amount []int) int {
	// step 0: rebuild graph as nodes instead of edges
	graph := make([]Node, len(amount))

	for _, edge := range edges {
		node := graph[edge[0]]
		node.index = edge[0]
		node.cost = amount[edge[0]]
		node.neighbours = append(node.neighbours, &graph[edge[1]])
		graph[edge[0]] = node

		node = graph[edge[1]]
		node.index = edge[1]
		node.cost = amount[edge[1]]
		node.neighbours = append(node.neighbours, &graph[edge[0]])
		graph[edge[1]] = node
	}

	// step 1: assume bob is required to take shortest path, find path from bob to node 0
	bobPath := findRootPath(bob, &graph, -1, make([]int, 0))

	// step 2: based on length of bob's path, update node values (0 for first half, half value if exactly in middle)
	for i := range len(bobPath) / 2 {
		graph[bobPath[i]].cost = 0
	}
	bobHalfIdx := (len(bobPath) - 1) / 2
	if len(bobPath)%2 == 1 {
		graph[bobPath[bobHalfIdx]].cost = graph[bobPath[bobHalfIdx]].cost / 2
	}

	// step 3: dfs from node 0 to each leaf node to find maximum profit
	mostProfit := profitToLeafNode(0, &graph, -1)

	return mostProfit
}

func findRootPath(node int, graph *[]Node, cameFrom int, curPath []int) []int {
	newPath := append(curPath, node)

	if node == 0 {
		return newPath
	}

	for _, neighbour := range (*graph)[node].neighbours {
		if neighbour.index != cameFrom {
			nextPath := findRootPath(neighbour.index, graph, node, newPath)
			if nextPath != nil {
				return nextPath
			}
		}
	}

	return nil
}

func profitToLeafNode(node int, graph *[]Node, cameFrom int) int {
	if len((*graph)[node].neighbours) == 1 && (*graph)[node].neighbours[0].index == cameFrom {
		// leaf node
		return (*graph)[node].cost
	}

	bestChild := math.MinInt32
	for _, neighbour := range (*graph)[node].neighbours {
		if neighbour.index != cameFrom {
			childValue := profitToLeafNode(neighbour.index, graph, node)
			if childValue > bestChild {
				bestChild = childValue
			}
		}
	}

	return (*graph)[node].cost + bestChild
}
