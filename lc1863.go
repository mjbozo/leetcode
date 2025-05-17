// LeetCode problem 1863: Sum of All Subset XOR Totals
// https://leetcode.com/problems/sum-of-all-subset-xor-totals/description/

package main

import "fmt"

func main() {
	result := subsetXORSum([]int{1, 3})
	fmt.Printf("Result = %d\n", result)

	result = subsetXORSum([]int{5, 1, 6})
	fmt.Printf("Result = %d\n", result)

	result = subsetXORSum([]int{3, 4, 5, 6, 7, 8})
	fmt.Printf("Result = %d\n", result)
}

func subsetXORSum(nums []int) int {
	sum := 0

	subsets := generateSubsets(nums, nums[0])

	for _, s := range subsets {
		current := 0
		if len(s) > 0 {
			current = s[0]
		}
		for i := 1; i < len(s); i++ {
			current ^= s[i]
		}

		sum += current
	}

	return sum
}

func generateSubsets(nums []int, current int) [][]int {
	subsets := make([][]int, 0)

	if len(nums) == 1 {
		subsets = append(subsets, []int{})
		subsets = append(subsets, []int{current})
	} else {
		rest := generateSubsets(nums[1:], nums[1])
		for _, r := range rest {
			subsets = append(subsets, r)

			s := make([]int, 0)
			s = append(s, current)
			s = append(s, r...)
			subsets = append(subsets, s)
		}
	}

	return subsets
}
