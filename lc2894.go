// LeetCode problem 2894: Divisible and Non-divisible Sums Difference
// https://leetcode.com/problems/divisible-and-non-divisible-sums-difference/description/

package main

import "fmt"

func main() {
	result := differenceOfSums(10, 3)
	fmt.Printf("Result = %d\n", result)
}

func differenceOfSums(n, m int) int {
	num1 := 0
	num2 := 0

	for i := 1; i <= n; i++ {
		if i%m == 0 {
			num2 += i
		} else {
			num1 += i
		}
	}

	return num1 - num2
}
