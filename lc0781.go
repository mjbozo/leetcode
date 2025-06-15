// LeetCode problem 0781: Rabbits in Forest
// https://leetcode.com/problems/rabbits-in-forest/description/

package main

import (
	"fmt"
	"math"
)

func main() {
	result := numRabbits([]int{0, 3, 2, 0, 3, 3, 4, 2, 4, 3, 2, 4, 4, 3, 0, 1, 3, 4, 4, 3})
	fmt.Printf("Result = %d\n", result)
}

func numRabbits(answers []int) int {
	uniqueAnswers := make(map[int]int)

	for _, answer := range answers {
		uniqueAnswers[answer]++
	}

	fmt.Println(uniqueAnswers)

	rabbits := 0
	for k, v := range uniqueAnswers {
		x := int(math.Ceil(float64(v) / float64(k+1)))
		rabbits += x * (k + 1)
	}

	return rabbits
}
