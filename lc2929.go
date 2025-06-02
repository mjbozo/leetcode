// LeetCode problem 2929: Distribute Candies Among Children II
// https://leetcode.com/problems/distribute-candies-among-children-ii/description/

package main

import "fmt"

func main() {
	result := distributeCandies(5, 2)
	fmt.Printf("Result = %d\n", result)
}

func distributeCandies(n int, limit int) int64 {
	var sum int64
	for i := range min(n, limit) + 1 {
		sum += int64(max(min(limit, n-i)-max(0, n-i-limit)+1, 0))
	}
	return sum
}
