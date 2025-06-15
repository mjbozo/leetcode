// LeetCode problem 1399: Count Largest Group
// https://leetcode.com/problems/count-largest-group/description/

package main

import "fmt"

func main() {
	result := countLargestGroup(13)
	fmt.Printf("Result = %d\n", result)
}

func countLargestGroup(n int) int {
	frequencyMap := make(map[int]int)
	largest := 0
	for i := 1; i <= n; i++ {
		sum := sumDigits(i)
		frequencyMap[sum]++
		largest = max(largest, frequencyMap[sum])
	}

	count := 0
	for _, v := range frequencyMap {
		if v == largest {
			count++
		}
	}

	return count
}

func sumDigits(n int) int {
	x := 0
	for n > 0 {
		x += n % 10
		n = n / 10
	}
	return x
}
