// LeetCode problem 3375: Minimum Operations to Make Array Values Equal to K
// https://leetcode.com/problems/minimum-operations-to-make-array-values-equal-to-k/description/

package main

import (
	"fmt"
	"slices"
)

func main() {
	result := minOperations([]int{5, 2, 5, 4, 5}, 2)
	fmt.Printf("Result = %d\n", result)

	result = minOperations([]int{2, 1, 2}, 2)
	fmt.Printf("Result = %d\n", result)

	result = minOperations([]int{9, 7, 5, 3}, 1)
	fmt.Printf("Result = %d\n", result)
}

func minOperations(nums []int, k int) int {
	slices.Sort(nums)

	if nums[0] < k {
		return -1
	}

	uniques := 1
	for i := 1; i < len(nums); i++ {
		if nums[i] != nums[i-1] {
			uniques++
		}
	}

	if k < nums[0] {
		return uniques
	}

	return uniques - 1
}
