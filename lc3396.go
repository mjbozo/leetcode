// LeetCode problem 3396: Minimum Number of Operations to Make Elements in Array Distinct
// https://leetcode.com/problems/minimum-number-of-operations-to-make-elements-in-array-distinct/description/

package main

import "fmt"

func main() {
	result := minimumOperations([]int{1, 2, 3, 4, 2, 3, 3, 5, 7})
	fmt.Printf("Result = %d\n", result)
}

func minimumOperations(nums []int) int {
	lastIndexes := make(map[int]int)
	lastToDelete := -3

	for i, n := range nums {
		if x, ok := lastIndexes[n]; ok && x > lastToDelete {
			lastToDelete = x
		}
		lastIndexes[n] = i
	}

	return (lastToDelete / 3) + 1
}
