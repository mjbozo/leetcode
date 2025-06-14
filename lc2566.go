// LeetCode problem 2566: Maximum Difference by Remapping a Digit
// https://leetcode.com/problems/maximum-difference-by-remapping-a-digit/description/

package main

import (
	"fmt"
	"strconv"
	"strings"
)

func main() {
	result := minMaxDifference(90)
	fmt.Printf("Result = %d\n", result)
}

func minMaxDifference(num int) int {
	numString := fmt.Sprintf("%d", num)

	var r rune
	for _, x := range numString {
		if x != '9' {
			r = x
			break
		}
	}

	maxString := strings.ReplaceAll(numString, string(r), "9")
	maxNum, _ := strconv.Atoi(maxString)

	minString := strings.ReplaceAll(numString, string(numString[0]), "0")
	minNum, _ := strconv.Atoi(minString)

	return maxNum - minNum
}
