// LeetCode problem 1298: Maximum Candies You Can Get from Boxes
// https://leetcode.com/problems/maximum-candies-you-can-get-from-boxes/description/

package main

import "fmt"

func main() {
	result := maxCandies([]int{1, 0, 1, 0}, []int{7, 5, 4, 100}, [][]int{[]int{}, []int{}, []int{1}, []int{}}, [][]int{[]int{1, 2}, []int{3}, []int{}, []int{}}, []int{0})
	fmt.Printf("Result = %d\n", result)
}

func maxCandies(status []int, candies []int, keys [][]int, containedBoxes [][]int, initialBoxes []int) int {
	numCandies := 0
	numBoxes := len(status)
	queue := make(chan int, numBoxes)
	for _, box := range initialBoxes {
		queue <- box
	}

	locked := make([]int, 0)
	prevLocked := make([]int, 0)
	for len(queue) > 0 {
		box := <-queue
		prevLocked = locked
		locked = make([]int, 0)

		if status[box] == 1 {
			numCandies += candies[box]
			for _, key := range keys[box] {
				status[key] = 1
			}
			for _, contained := range containedBoxes[box] {
				queue <- contained
			}
		} else {
			locked = append(locked, box)
		}

		if len(locked) != 0 && equal(locked, prevLocked) {
			// stuck in same loop?
			break
		}

		for _, lock := range locked {
			queue <- lock
		}
	}

	return numCandies
}

func equal(a, b []int) bool {
	if len(a) != len(b) {
		return false
	}

	for i := 0; i < len(a); i++ {
		if a[i] != b[i] {
			return false
		}
	}

	return true
}
