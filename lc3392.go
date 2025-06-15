// LeetCode problem 3392: Count Subarrays of Length Three With a Condition
// https://leetcode.com/problems/count-subarrays-of-length-three-with-a-condition/description/

package main

import "fmt"

func main() {
	result := countSubarrays([]int{1, 2, 1, 4, 1})
	fmt.Printf("Result = %d\n", result)
}

func countSubarrays(nums []int) int {
	n := len(nums)
	count := 0
	for i := 0; i < n-2; i++ {
		if (nums[i]+nums[i+2])*2 == nums[i+1] {
			count++
		}
	}
	return count
}
