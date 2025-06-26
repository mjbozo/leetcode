// LeetCode problem 2200: Find All K-Distant Indices in an Array
// https://leetcode.com/problems/find-all-k-distant-indices-in-an-array/description/

package main

import "fmt"

func main() {
	result := findKDistantIndices([]int{3, 4, 9, 1, 3, 9, 5}, 9, 1)
	fmt.Printf("Result = %v\n", result)
}

func findKDistantIndices(nums []int, key int, k int) []int {
	n := len(nums)
	res := make([]int, 0, n)

	for i := 0; i < n; i++ {
		for j := max(0, i-k); j < min(n-1, i+k); j++ {
			if nums[j] == key {
				res = append(res, i)
				break
			}
		}
	}

	return res
}
