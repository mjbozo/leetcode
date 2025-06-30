// LeetCode problem 2099: Find Subsequence of Length K With the Largest Sum
// https://leetcode.com/problems/find-subsequence-of-length-k-with-the-largest-sum/description/

package main

import (
	"fmt"
	"slices"
)

type Pair struct {
	idx int
	num int
}

func main() {
	result := maxSubsequence([]int{2, 1, 3, 3}, 2)
	fmt.Printf("Result = %v\n", result)
}

func maxSubsequence(nums []int, k int) []int {
	n := len(nums)
	numsAndIndexes := make([]Pair, 0, n)
	for i, n := range nums {
		numsAndIndexes = append(numsAndIndexes, Pair{idx: i, num: n})
	}

	slices.SortFunc(numsAndIndexes, func(a, b Pair) int {
		return b.num - a.num
	})

	sub := numsAndIndexes[:k]
	res := make([]int, 0, k)

	slices.SortFunc(sub, func(a, b Pair) int {
		return a.idx - b.idx
	})

	for _, x := range sub {
		res = append(res, x.num)
	}

	return res
}
