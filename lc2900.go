// LeetCode problem 2900: Longest Unequal Adjacent Groups Subsequence I
// https://leetcode.com/problems/longest-unequal-adjacent-groups-subsequence/description/

package main

import "fmt"

func main() {
	result := getLongestSubsequence([]string{"e", "a", "b"}, []int{0, 0, 1})
	fmt.Printf("Result = %v\n", result)
}

func getLongestSubsequence(words []string, groups []int) []string {
	res := make([]string, 0)
	res = append(res, words[0])

	last := groups[0]

	for i := 1; i < len(groups); i++ {
		x := groups[i]
		if x != last {
			res = append(res, words[i])
			last = x
		}
	}

	return res
}
