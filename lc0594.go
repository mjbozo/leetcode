// LeetCode problem 0594: Longest Harmonious Subsequence
// https://leetcode.com/problems/longest-harmonious-subsequence/description/

package main

import (
	"fmt"
	"slices"
)

func main() {
	result := findLHS([]int{1, 3, 2, 2, 5, 2, 3, 7})
	fmt.Printf("Result = %d\n", result)
}

func findLHS(nums []int) int {
	n := len(nums)
	slices.Sort(nums)

	longest := 0
	left := 0
	right := 1

	for right < n {
		if nums[right]-nums[left] > 1 {
			left++
		}

		if nums[right]-nums[left] < 1 {
			right++
		}

		for right < n && nums[right]-nums[left] == 1 {
			longest = max(longest, right-left+1)
			right++
		}
	}

	return longest
}
