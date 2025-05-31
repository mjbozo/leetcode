// LeetCode problem 2359: Find Closest Node to Given Two Nodes
// https://leetcode.com/problems/find-closest-node-to-given-two-nodes/description/

package main

import "fmt"

func main() {
	result := closestMeetingNode([]int{2, 2, 3, -1}, 0, 1)
	fmt.Printf("Result = %v\n", result)
}

func closestMeetingNode(edges []int, node1 int, node2 int) int {
	n := len(edges)
	node1Distances := make([]int, n)
	for i := 0; i < n; i++ {
		node1Distances[i] = -1
	}

	distance := 0
	for node1 != -1 {
		if node1Distances[node1] != -1 {
			// loop detected
			break
		}

		node1Distances[node1] = distance
		node1 = edges[node1]
		distance++
	}

	node2Distances := make([]int, n)
	for i := 0; i < n; i++ {
		node2Distances[i] = -1
	}

	distance = 0
	for node2 != -1 {
		if node2Distances[node2] != -1 {
			// loop detected
			break
		}

		node2Distances[node2] = distance
		node2 = edges[node2]
		distance++
	}

	closest := n
	closestIndex := -1
	for i := 0; i < n; i++ {
		if node1Distances[i] != -1 && node2Distances[i] != -1 {
			longest := max(node1Distances[i], node2Distances[i])
			if longest < closest {
				closest = longest
				closestIndex = i
			}
		}
	}

	return closestIndex
}
