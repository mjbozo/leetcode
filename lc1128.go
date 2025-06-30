// LeetCode problem 1128: Number of Equivalent Domino Pairs
// https://leetcode.com/problems/number-of-equivalent-domino-pairs/description/

package main

import "fmt"

type Pair struct {
	a int
	b int
}

func main() {
	result := numEquivDominoPairs([][]int{[]int{1, 2}, []int{2, 1}, []int{3, 4}, []int{5, 6}})
	fmt.Printf("Result = %v\n", result)
}

func numEquivDominoPairs(dominoes [][]int) int {
	pairs := make(map[Pair]int)
	for _, dom := range dominoes {
		a := min(dom[0], dom[1])
		b := max(dom[0], dom[1])
		pairs[Pair{a: a, b: b}]++
	}

	equivalentPairs := 0
	for _, v := range pairs {
		equivalentPairs += (v * (v - 1)) / 2
	}

	return equivalentPairs
}
