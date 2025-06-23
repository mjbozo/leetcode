// LeetCode problem 3085: Minimum Deletions to Make String K-Special
// https://leetcode.com/problems/minimum-deletions-to-make-string-k-special/description/

package main

import "fmt"

func main() {
	result := minimumDeletions("aabcaba", 0)
	fmt.Printf("Result = %d\n", result)
}

func minimumDeletions(word string, k int) int {
	freq := make([]int, 26)
	for _, r := range word {
		freq[r-'a']++
	}

	letters := make([]int, 0)
	for _, f := range freq {
		if f > 0 {
			letters = append(letters, f)
		}
	}

	best := len(word)

	for i, v := range letters {
		current := 0
		for j, w := range letters {
			if i == j {
				continue
			}

			if w < v {
				current += w
			} else if w > v+k {
				current += w - v - k
			}
		}
		best = min(best, current)
	}

	return best
}
