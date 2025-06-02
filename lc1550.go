// LeetCode problem 1550: Three Consecutive Odds
// https://leetcode.com/problems/three-consecutive-odds/description/

package main

import "fmt"

func main() {
	result := threeConsecutiveOdds([]int{1, 2, 34, 3, 4, 5, 7, 23, 12})
	fmt.Printf("Result = %t\n", result)
}

func threeConsecutiveOdds(arr []int) bool {
	oddCount := 0
	for _, n := range arr {
		if n%2 == 1 {
			oddCount++
			if oddCount == 3 {
				return true
			}
		} else {
			oddCount = 0
		}
	}
	return false
}
