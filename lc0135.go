// LeetCode problem 0135: Candy
// https://leetcode.com/problems/candy/description/

package main

import "fmt"

func main() {
	ratings := []int{0, 1, 2, 3, 40, 25, 50, 23, 22, 43, 0, 1, 0, 2, 3, 3, 5, 5, 8}
	result := candy(ratings)
	fmt.Printf("Result = %d\n", result)
}

func candy(ratings []int) int {
	n := len(ratings)
	candies := make([]int, n)
	for i := 0; i < n; i++ {
		candies[i] = 1
	}

	for i := 1; i < n; i++ {
		if ratings[i] > ratings[i-1] {
			candies[i] = candies[i-1] + 1
		}
	}

	for i := n - 2; i >= 0; i-- {
		if ratings[i] > ratings[i+1] && candies[i] <= candies[i+1] {
			candies[i] = candies[i+1] + 1
		}
	}

	sum := 0
	for _, candy := range candies {
		sum += candy
	}

	return sum
}
