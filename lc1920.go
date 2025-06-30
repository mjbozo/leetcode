// LeetCode problem 1920: Build Array from Permutation
// https://leetcode.com/problems/build-array-from-permutation/description/

package main

import "fmt"

func main() {
	result := buildArray([]int{0, 2, 1, 5, 3, 4})
	fmt.Printf("Result = %v\n", result)
}

func buildArray(nums []int) []int {
	n := len(nums)
	res := make([]int, 0, n)
	for i := range n {
		res = append(res, nums[nums[i]])
	}
	return res
}
