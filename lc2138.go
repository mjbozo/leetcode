// LeetCode problem 2138: Divide a String Into Groups of Size K
// https://leetcode.com/problems/divide-a-string-into-groups-of-size-k/description/

package main

import (
	"fmt"
	"strings"
)

func main() {
	result := divideString("abcdefghij", 3, 'x')
	fmt.Printf("Result = %d\n", result)
}

func divideString(s string, k int, fill byte) []string {
	n := len(s)

	if k >= n {
		return []string{s + strings.Repeat(string(fill), k-n)}
	}

	s += strings.Repeat(string(fill), k%n)

	res := make([]string, 0)
	for i := 0; i < n; i += k {
		res = append(res, s[i:i+k])
	}

	return res
}
