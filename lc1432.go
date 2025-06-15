// LeetCode problem 1432: Max Difference You Can Get From Changing an Integer
// https://leetcode.com/problems/max-difference-you-can-get-from-changing-an-integer/description/

package main

import (
	"fmt"
	"strconv"
	"strings"
)

func main() {
	result := maxDiff(11011057)
	fmt.Printf("Result = %d\n", result)
}

func maxDiff(num int) int {
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

	minReplace := "1"
	for i, x := range numString {
		if x != '1' && x != '0' {
			r = x
			if i > 0 {
				minReplace = "0"
			}
			break
		}
	}

	minString := strings.ReplaceAll(numString, string(r), minReplace)
	minNum, _ := strconv.Atoi(minString)

	return maxNum - minNum
}
