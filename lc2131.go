// LeetCode problem 2131: Longest Palindrome by Concatenating Two Letter Words
// https://leetcode.com/problems/longest-palindrome-by-concatenating-two-letter-words/description/

package main

import "fmt"

func main() {
	result := longestPalindrome([]string{"lc", "cl", "gg", "gg", "cc", "cl"})
	fmt.Printf("Result = %d\n", result)
}

func longestPalindrome(words []string) int {
	longest := 0
	doubles := 0
	matches := make([]int, 26*26)

	for _, word := range words {
		x := word[0]
		y := word[1]

		if x == y {
			doubles++
		}

		invIndex := getPairIndex(y, x)
		if matches[invIndex] > 0 {
			matches[invIndex]--
			longest += 4

			if x == y {
				doubles -= 2
			}
		} else {
			matches[getPairIndex(x, y)]++
		}
	}

	if doubles > 0 {
		longest += 2
	}

	return longest
}

func getPairIndex(a, b byte) int {
	return (int(a-'a') * 26) + int(b-'a')
}
