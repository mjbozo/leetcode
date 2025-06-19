// LeetCode problem 2294: Partition Array Such That Maximum Difference Is K
// https://leetcode.com/problems/partition-array-such-that-maximum-difference-is-k/description/

package main

import (
	"fmt"
	"slices"
)

func main() {
	result := partitionArray([]int{3, 6, 1, 2, 5}, 2)
	fmt.Printf("Result = %d\n", result)
}

func partitionArray(nums []int, k int) int {
	slices.Sort(nums)
	current := nums[0]
	partitions := 1

	for i := 1; i < len(nums); i++ {
		if nums[i] > current+k {
			partitions++
			current = nums[i]
		}
	}

	return partitions
}
