// LeetCode problem 2016: Maximum Difference Between Increasing Elements
// https://leetcode.com/problems/maximum-difference-between-increasing-elements/description/

package main

import "fmt"

func main() {
	result := maximumDifference([]int{7, 1, 5, 4})
	fmt.Printf("Result = %d\n", result)
}

func maximumDifference(nums []int) int {
	res := -1
	lower := nums[0]
	upper := 0

	for _, x := range nums {
		upper = max(upper, x)
		if x < lower {
			lower = x
			upper = 0
		}
		if upper > lower {
			res = max(res, upper-lower)
		}
	}

	return res
}
