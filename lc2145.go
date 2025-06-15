// LeetCode problem 2145: Count the Hidden Sequences
// https://leetcode.com/problems/count-the-hidden-sequences/description/

package main

import "fmt"

func main() {
	result := numberOfArrays([]int{1, -3, 4}, 1, 6)
	fmt.Printf("Result = %d\n", result)
}

func numberOfArrays(differences []int, lower int, upper int) int {
	current := 0
	minimum := 0
	maximum := 0

	for _, x := range differences {
		current += x
		minimum = min(minimum, current)
		maximum = max(maximum, current)
	}

	return max(0, upper-(lower+maximum-minimum)+1)
}
